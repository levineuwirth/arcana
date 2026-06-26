//! `arcana-web` — an MVP web/server GUI for playing a game of Arcana against a
//! bot in the browser. The first step toward an MTGA/MTGO-style frontend.
//!
//! All game logic lives in the library crate ([`arcana_web`]); this binary is
//! just the axum/tokio HTTP shell around it:
//!
//! * `GET  /`       → the single-page game UI (`static/index.html`, embedded).
//! * `GET  /deck`   → the deckbuilder UI (`static/deck.html`, embedded).
//! * `GET  /glossary`→ keyword reminder text (base name → reminder), static.
//! * `GET  /state`  → advance through bot/trivial decisions, return the human's
//!   [`ViewState`](arcana_core::view::ViewState) + the opponent action log.
//! * `POST /action` → body `{ "index": N }`, apply `legal[N]`, advance, return
//!   the new state. Out-of-range `N` → 400.
//! * `POST /combat` → body `{ "kind":"attackers", "attackers":[…] }` or
//!   `{ "kind":"blockers", "blockers":[…] }`, the incremental combat builder's
//!   declaration; matched against the legal enumeration. Illegal set → 400.
//! * `POST /autotap`→ body `{ "object_id": N }`, MTGA-style "click to play":
//!   auto-tap mana for hand card N and cast/play it (or float mana and surface
//!   the cast variants if a choice remains). Not playable → 400.
//! * `POST /bottom` → body `{ "ids": [N, …] }`, the London-mulligan bottoming:
//!   put the chosen cards on the bottom of the library. Malformed → 400.
//! * `POST /search` → body `CardQuery` (name/colors/types/keywords/cmc/limit), the
//!   catalog query for deckbuilding; returns `[CardInfo, …]` (capped at 200).
//! * `POST /new`    → optional body `{ "seed": N }`, start a fresh game.
//!
//! ## Concurrency / lifetime note
//! `Session<'a>` borrows the `CardRegistry`, and `Seat::Bot(Box<dyn StatePolicy>)`
//! is `!Send`, so the live game can't be shared directly in axum's
//! `Send + Sync` router state. Instead a single dedicated worker thread OWNS the
//! [`GameCore`] (building and leaking the registry to `'static` locally, as the
//! single-process design calls for) and the async handlers talk to it over
//! channels — only plain-data [`StateResponse`]s and command messages cross
//! threads. This keeps all game state on one thread (no locking races) and
//! satisfies the framework's bounds without touching the engine crates.

use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use arcana_core::catalog::{CardInfo, CardQuery};
use arcana_core::objects::ObjectId;
use arcana_core::registry::CardRegistry;
use arcana_core::types::CardId;
use arcana_web::{CombatSubmission, GameCore, StateResponse};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

/// The single-page UI, embedded so the binary is self-contained.
const INDEX_HTML: &str = include_str!("../static/index.html");
/// The deckbuilder page (browse the catalog, build a deck, play it).
const DECK_HTML: &str = include_str!("../static/deck.html");

/// A request to the game worker thread. Each carries a oneshot reply channel.
enum Command {
    State(oneshot::Sender<StateResponse>),
    Action { index: usize, reply: oneshot::Sender<Result<StateResponse, String>> },
    Combat { sub: CombatSubmission, reply: oneshot::Sender<Result<StateResponse, String>> },
    AutoTap { target: ObjectId, reply: oneshot::Sender<Result<StateResponse, String>> },
    Bottom { ids: Vec<ObjectId>, reply: oneshot::Sender<Result<StateResponse, String>> },
    Search { query: CardQuery, reply: oneshot::Sender<Vec<CardInfo>> },
    New { seed: Option<u64>, deck: Option<Vec<CardId>>, reply: oneshot::Sender<StateResponse> },
}

/// Shared server state: a handle to the game worker. Cheap to clone (just an
/// mpsc sender), and `Send + Sync + 'static` as axum requires.
#[derive(Clone)]
struct AppState {
    tx: mpsc::UnboundedSender<Command>,
}

#[derive(Debug, Deserialize)]
struct ActionRequest {
    index: usize,
}

#[derive(Debug, Deserialize)]
struct AutoTapRequest {
    object_id: ObjectId,
}

#[derive(Debug, Deserialize)]
struct BottomRequest {
    ids: Vec<ObjectId>,
}

#[derive(Debug, Default, Deserialize)]
struct NewRequest {
    seed: Option<u64>,
    /// Optional custom deck (card ids with repeats, from the deckbuilder). Both
    /// seats play it. Ignored if too small or containing unregistered ids.
    deck: Option<Vec<CardId>>,
}

/// A deck is playable if it can at least draw an opening hand and every id is a
/// registered card (a bad submission falls back to the sample deck rather than
/// risking an engine panic).
fn deck_is_valid(reg: &CardRegistry, deck: &[CardId]) -> bool {
    deck.len() >= 7 && deck.iter().all(|&id| reg.get(id).is_some())
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

fn err(msg: impl Into<String>) -> Json<ErrorBody> {
    Json(ErrorBody { error: msg.into() })
}

/// A semi-fresh seed for "new game" when the client doesn't pin one.
fn time_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

/// The worker thread's main loop: owns one [`GameCore`] and serves commands
/// sequentially. The registry is built and leaked to `'static` here, so it
/// never has to cross a thread boundary.
fn run_worker(mut rx: mpsc::UnboundedReceiver<Command>) {
    let reg: &'static CardRegistry = Box::leak(Box::new(arcana_cards::build_catalog()));
    let mut core = GameCore::new(reg, time_seed());

    while let Some(cmd) = rx.blocking_recv() {
        match cmd {
            Command::State(reply) => {
                let _ = reply.send(core.snapshot());
            }
            Command::Action { index, reply } => {
                let res = core.apply_index(index).map_err(|e| e.to_string());
                let _ = reply.send(res);
            }
            Command::Combat { sub, reply } => {
                let res = core.apply_combat(sub).map_err(|e| e.to_string());
                let _ = reply.send(res);
            }
            Command::AutoTap { target, reply } => {
                let res = core.auto_tap_and_cast(target).map_err(|e| e.to_string());
                let _ = reply.send(res);
            }
            Command::Bottom { ids, reply } => {
                let res = core.bottom_cards(ids).map_err(|e| e.to_string());
                let _ = reply.send(res);
            }
            Command::Search { query, reply } => {
                let _ = reply.send(arcana_core::catalog::query(core.registry(), &query));
            }
            Command::New { seed, deck, reply } => {
                let seed = seed.unwrap_or_else(time_seed);
                core = match deck {
                    Some(d) if deck_is_valid(reg, &d) => GameCore::new_with_deck(reg, seed, d),
                    _ => GameCore::new(reg, seed),
                };
                let _ = reply.send(core.snapshot());
            }
        }
    }
}

/// Lost-worker fallback (the worker thread should outlive the server).
fn worker_gone() -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, err("game worker is unavailable")).into_response()
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn deckbuilder() -> Html<&'static str> {
    Html(DECK_HTML)
}

/// The keyword glossary (base name -> reminder text). Static — no game state, so
/// the frontend fetches it once and caches it for the card-zoom chips.
async fn get_glossary() -> Json<std::collections::BTreeMap<String, String>> {
    Json(
        arcana_core::glossary::keyword_glossary()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    )
}

async fn get_state(State(app): State<AppState>) -> Response {
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::State(reply)).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(resp) => Json(resp).into_response(),
        Err(_) => worker_gone(),
    }
}

async fn post_action(State(app): State<AppState>, body: String) -> Response {
    let req: ActionRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, err(format!("invalid /action body: {e}"))).into_response()
        }
    };
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::Action { index: req.index, reply }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(Ok(resp)) => Json(resp).into_response(),
        Ok(Err(msg)) => (StatusCode::BAD_REQUEST, err(msg)).into_response(),
        Err(_) => worker_gone(),
    }
}

async fn post_combat(State(app): State<AppState>, body: String) -> Response {
    let sub: CombatSubmission = match serde_json::from_str(&body) {
        Ok(s) => s,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, err(format!("invalid /combat body: {e}"))).into_response()
        }
    };
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::Combat { sub, reply }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(Ok(resp)) => Json(resp).into_response(),
        Ok(Err(msg)) => (StatusCode::BAD_REQUEST, err(msg)).into_response(),
        Err(_) => worker_gone(),
    }
}

async fn post_autotap(State(app): State<AppState>, body: String) -> Response {
    let req: AutoTapRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, err(format!("invalid /autotap body: {e}"))).into_response()
        }
    };
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::AutoTap { target: req.object_id, reply }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(Ok(resp)) => Json(resp).into_response(),
        Ok(Err(msg)) => (StatusCode::BAD_REQUEST, err(msg)).into_response(),
        Err(_) => worker_gone(),
    }
}

async fn post_bottom(State(app): State<AppState>, body: String) -> Response {
    let req: BottomRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, err(format!("invalid /bottom body: {e}"))).into_response()
        }
    };
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::Bottom { ids: req.ids, reply }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(Ok(resp)) => Json(resp).into_response(),
        Ok(Err(msg)) => (StatusCode::BAD_REQUEST, err(msg)).into_response(),
        Err(_) => worker_gone(),
    }
}

async fn post_search(State(app): State<AppState>, body: String) -> Response {
    // Lenient: empty body is an unconstrained query. Cap results for the UI
    // unless the client asked for a specific limit (the catalog is huge).
    let mut query: CardQuery = serde_json::from_str(&body).unwrap_or_default();
    if query.limit.is_none() {
        query.limit = Some(200);
    }
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::Search { query, reply }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(results) => Json(results).into_response(),
        Err(_) => worker_gone(),
    }
}

async fn post_new(State(app): State<AppState>, body: String) -> Response {
    // Lenient: empty body is allowed (→ default → time-based seed, sample deck).
    let req: NewRequest = serde_json::from_str(&body).unwrap_or_default();
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::New { seed: req.seed, deck: req.deck, reply }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(resp) => Json(resp).into_response(),
        Err(_) => worker_gone(),
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::unbounded_channel::<Command>();
    // The game lives on its own thread; building the catalog (and leaking it)
    // happens there. `advance` runs the bot, so this also keeps the bot's work
    // off the async runtime's worker threads.
    std::thread::Builder::new()
        .name("arcana-game".into())
        .spawn(move || run_worker(rx))
        .expect("spawn game worker");

    let app = Router::new()
        .route("/", get(index))
        .route("/deck", get(deckbuilder))
        .route("/glossary", get(get_glossary))
        .route("/state", get(get_state))
        .route("/action", post(post_action))
        .route("/combat", post(post_combat))
        .route("/autotap", post(post_autotap))
        .route("/bottom", post(post_bottom))
        .route("/search", post(post_search))
        .route("/new", post(post_new))
        .with_state(AppState { tx });

    // Port is overridable via PORT for convenience; defaults to 8080.
    let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {addr}: {e}"));
    println!("arcana-web listening on http://{addr}  (open it in a browser)");
    axum::serve(listener, app).await.expect("server error");
}
