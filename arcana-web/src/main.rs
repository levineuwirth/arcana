//! `arcana-web` — an MVP web/server GUI for playing a game of Arcana against a
//! bot in the browser. The first step toward an MTGA/MTGO-style frontend.
//!
//! All game logic lives in the library crate ([`arcana_web`]); this binary is
//! just the axum/tokio HTTP shell around it:
//!
//! * `GET  /`       → the single-page game UI (`static/index.html`, embedded).
//! * `GET  /deck`   → the deckbuilder UI (`static/deck.html`, embedded).
//! * `GET  /theme.css` → the shared design-token stylesheet (themes, mana palette).
//! * `GET  /app.js` → the shared frontend module (card component, theme, Api).
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
//! * `GET  /formats`→ built-in `[FormatSpec]` for the legality selector.
//! * `POST /import` → body `{ "text": "<deck list>" }`, parse Arena/MTGO format →
//!   resolved `ImportedDeck` (main/sideboard CardInfos + unresolved names).
//! * `POST /legality`→ body `{ main, sideboard, spec }` → `LegalityReport`.
//! * `GET  /sample-deck` → the baseline sample deck as `[CardId]` (opponent picker).
//! * `POST /new`    → optional `{ seed, deck, opponent }`; human plays `deck`, bot
//!   plays `opponent` (mirrors `deck` if absent). Invalid decks fall back to sample.
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
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use arcana_core::catalog::{CardInfo, CardQuery};
use arcana_core::deck::{check_legality, builtin_formats, FormatSpec, LegalityReport};
use arcana_core::objects::ObjectId;
use arcana_core::registry::CardRegistry;
use arcana_core::types::CardId;
use arcana_ai::session::AutoPass;
use arcana_web::{resolve_import, CombatSubmission, GameCore, ImportedDeck, StateResponse, Suggestion};
use axum::extract::{Query, State};
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
/// Shared design-token stylesheet (themes, type, mana palette, card component).
const THEME_CSS: &str = include_str!("../static/theme.css");
/// Shared frontend module (theme switcher, card component, zoom, Api client).
const APP_JS: &str = include_str!("../static/app.js");

/// A request to the game worker thread. Each carries a oneshot reply channel.
enum Command {
    State(oneshot::Sender<StateResponse>),
    Action { index: usize, reply: oneshot::Sender<Result<StateResponse, String>> },
    Combat { sub: CombatSubmission, reply: oneshot::Sender<Result<StateResponse, String>> },
    AutoTap { target: ObjectId, reply: oneshot::Sender<Result<StateResponse, String>> },
    Activate { source: ObjectId, reply: oneshot::Sender<Result<StateResponse, String>> },
    SetAutoPass { level: AutoPass, reply: oneshot::Sender<StateResponse> },
    AllCardNames { reply: oneshot::Sender<Vec<String>> },
    Bottom { ids: Vec<ObjectId>, reply: oneshot::Sender<Result<StateResponse, String>> },
    Search { query: CardQuery, reply: oneshot::Sender<Vec<CardInfo>> },
    Import { text: String, reply: oneshot::Sender<ImportedDeck> },
    Legality {
        main: Vec<(CardId, u32)>,
        sideboard: Vec<(CardId, u32)>,
        spec: FormatSpec,
        reply: oneshot::Sender<LegalityReport>,
    },
    SampleDeck { reply: oneshot::Sender<Vec<CardId>> },
    Suggest { deep: bool, reply: oneshot::Sender<Vec<Suggestion>> },
    New {
        seed: Option<u64>,
        deck: Option<Vec<CardId>>,
        opponent: Option<Vec<CardId>>,
        reply: oneshot::Sender<StateResponse>,
    },
}

/// Shared server state: a handle to the game worker. Cheap to clone (just an
/// mpsc sender + an `Arc`), and `Send + Sync + 'static` as axum requires.
#[derive(Clone)]
struct AppState {
    tx: mpsc::UnboundedSender<Command>,
    art: Arc<ArtCache>,
}

/// Card-art proxy with a persistent on-disk cache. The browser requests
/// `/art?name=…` from us instead of hammering Scryfall's rate-limited API
/// directly (which fails progressively as you browse). We serve a cached image
/// instantly, or fetch it from Scryfall ONCE — throttled to a polite cadence —
/// and cache it to disk so it persists across sessions (and works offline after).
struct ArtCache {
    dir: PathBuf,
    client: reqwest::Client,
    /// Serializes outbound Scryfall fetches to a minimum spacing (their API asks
    /// for ~50–100ms between requests). Cache hits don't touch this.
    gate: tokio::sync::Mutex<Instant>,
    /// Progress of a "download all art" background job (one at a time).
    warm: WarmState,
}

/// Live progress of the bulk art download (see `post_art_warm_all`).
#[derive(Default)]
struct WarmState {
    running: AtomicBool,
    total: AtomicUsize,
    done: AtomicUsize,
    failed: AtomicUsize,
}

#[derive(Serialize)]
struct WarmStatus {
    running: bool,
    total: usize,
    done: usize,
    failed: usize,
}

impl ArtCache {
    fn new() -> Self {
        let dir = std::env::var("ARCANA_ART_CACHE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| match std::env::var("HOME") {
                Ok(h) => PathBuf::from(h).join(".cache/arcana/art"),
                Err(_) => std::env::temp_dir().join("arcana-art"),
            });
        let _ = std::fs::create_dir_all(&dir);
        let client = reqwest::Client::builder()
            .user_agent("Arcana/0.1 (card-art proxy)")
            .build()
            .expect("reqwest client");
        Self { dir, client, gate: tokio::sync::Mutex::new(Instant::now()), warm: WarmState::default() }
    }

    fn warm_status(&self) -> WarmStatus {
        WarmStatus {
            running: self.warm.running.load(Ordering::Relaxed),
            total: self.warm.total.load(Ordering::Relaxed),
            done: self.warm.done.load(Ordering::Relaxed),
            failed: self.warm.failed.load(Ordering::Relaxed),
        }
    }

    fn path_for(&self, name: &str) -> PathBuf {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        name.to_lowercase().hash(&mut h);
        self.dir.join(format!("{:016x}.jpg", h.finish()))
    }

    /// Cached image bytes for `name`, fetching + caching from Scryfall on a miss.
    /// `None` if the fetch fails (the client then shows its art fallback).
    async fn fetch(&self, name: &str) -> Option<Vec<u8>> {
        let path = self.path_for(name);
        if let Ok(bytes) = tokio::fs::read(&path).await {
            return Some(bytes);
        }
        // Throttle outbound Scryfall requests (hold the gate across the spacing
        // sleep so all fetches stay ≥90ms apart, ≤~11/s).
        {
            let mut last = self.gate.lock().await;
            const MIN: Duration = Duration::from_millis(90);
            let since = last.elapsed();
            if since < MIN {
                tokio::time::sleep(MIN - since).await;
            }
            *last = Instant::now();
        }
        let resp = self.client
            .get("https://api.scryfall.com/cards/named")
            .query(&[("format", "image"), ("exact", name)])
            .send()
            .await
            .ok()?;
        if !resp.status().is_success() {
            return None;
        }
        let bytes = resp.bytes().await.ok()?.to_vec();
        let _ = tokio::fs::write(&path, &bytes).await;
        Some(bytes)
    }
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

#[derive(Debug, Deserialize)]
struct ImportRequest {
    text: String,
}

/// `/suggest` query: `?deep=true` runs the heavier "deepen" budget; omitted or
/// `?deep=false` runs the snappy auto budget.
#[derive(Debug, Deserialize)]
struct SuggestQuery {
    #[serde(default)]
    deep: bool,
}

#[derive(Debug, Deserialize)]
struct LegalityRequest {
    #[serde(default)]
    main: Vec<(CardId, u32)>,
    #[serde(default)]
    sideboard: Vec<(CardId, u32)>,
    spec: FormatSpec,
}

#[derive(Debug, Default, Deserialize)]
struct NewRequest {
    seed: Option<u64>,
    /// The human's deck (seat 0). Ignored if too small / has unregistered ids.
    deck: Option<Vec<CardId>>,
    /// The opponent's deck (seat 1). If absent, the bot mirrors the human's deck.
    opponent: Option<Vec<CardId>>,
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
    // Persists across `New` so the player's auto-pass choice survives a new game.
    let mut auto_pass = AutoPass::default();
    core.set_auto_pass(auto_pass);

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
            Command::Activate { source, reply } => {
                let res = core.auto_tap_and_activate(source).map_err(|e| e.to_string());
                let _ = reply.send(res);
            }
            Command::SetAutoPass { level, reply } => {
                auto_pass = level;
                core.set_auto_pass(level);
                let _ = reply.send(core.snapshot());
            }
            Command::AllCardNames { reply } => {
                let r = core.registry();
                let mut names: Vec<String> = r.iter()
                    .filter_map(|(_, def)| r.interner().resolve(def.name).map(|s| s.to_string()))
                    .filter(|s| !s.is_empty())
                    .collect();
                names.sort();
                names.dedup();
                let _ = reply.send(names);
            }
            Command::Bottom { ids, reply } => {
                let res = core.bottom_cards(ids).map_err(|e| e.to_string());
                let _ = reply.send(res);
            }
            Command::Search { query, reply } => {
                let _ = reply.send(arcana_core::catalog::query(core.registry(), &query));
            }
            Command::Import { text, reply } => {
                let _ = reply.send(resolve_import(core.registry(), &text));
            }
            Command::Legality { main, sideboard, spec, reply } => {
                let _ = reply.send(check_legality(&main, &sideboard, &spec, core.registry()));
            }
            Command::SampleDeck { reply } => {
                let _ = reply.send(arcana_cards::sample_deck(reg, arcana_web::DECK_SEED));
            }
            Command::Suggest { deep, reply } => {
                let _ = reply.send(core.suggest(deep));
            }
            Command::New { seed, deck, opponent, reply } => {
                let seed = seed.unwrap_or_else(time_seed);
                core = match deck.filter(|d| deck_is_valid(reg, d)) {
                    Some(human) => {
                        // Bot plays the chosen opponent deck, else mirrors the human.
                        let bot = opponent.filter(|d| deck_is_valid(reg, d))
                            .unwrap_or_else(|| human.clone());
                        GameCore::new_with_decks(reg, seed, human, bot)
                    }
                    None => GameCore::new(reg, seed), // sample mirror
                };
                core.set_auto_pass(auto_pass); // preserve the player's choice
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

/// The shared design-token stylesheet, served with the right content-type so the
/// browser caches/parses it as CSS (both pages `<link>` it).
async fn theme_css() -> Response {
    ([(axum::http::header::CONTENT_TYPE, "text/css; charset=utf-8")], THEME_CSS).into_response()
}

/// The shared frontend module (card component, theme switcher, zoom, Api client).
async fn app_js() -> Response {
    (
        [(axum::http::header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
        APP_JS,
    )
        .into_response()
}

/// The built-in deck formats (static — the UI populates a selector and the legality
/// engine treats built-in and user-defined specs identically).
async fn get_formats() -> Json<Vec<FormatSpec>> {
    Json(builtin_formats())
}

async fn post_import(State(app): State<AppState>, body: String) -> Response {
    let req: ImportRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, err(format!("invalid /import body: {e}"))).into_response()
        }
    };
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::Import { text: req.text, reply }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(deck) => Json(deck).into_response(),
        Err(_) => worker_gone(),
    }
}

async fn post_legality(State(app): State<AppState>, body: String) -> Response {
    let req: LegalityRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, err(format!("invalid /legality body: {e}"))).into_response()
        }
    };
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::Legality {
        main: req.main, sideboard: req.sideboard, spec: req.spec, reply,
    }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(report) => Json(report).into_response(),
        Err(_) => worker_gone(),
    }
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

/// Ranked suggested lines for the human's current decision (cockpit panel).
/// Returns `[]` (200) when there is no real choice; the value-MC rollouts run on
/// the worker thread, so a heavy `?deep=true` request briefly blocks other
/// commands (acceptable for the single-user local cockpit; noted for multiplayer).
async fn get_suggest(State(app): State<AppState>, Query(q): Query<SuggestQuery>) -> Response {
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::Suggest { deep: q.deep, reply }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(s) => Json(s).into_response(),
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

#[derive(Debug, Deserialize)]
struct AutoPassRequest {
    level: String,
}

#[derive(Debug, Deserialize)]
struct ArtRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
struct WarmRequest {
    names: Vec<String>,
}

#[derive(Debug, Serialize)]
struct WarmResponse {
    requested: usize,
    ok: usize,
    failed: usize,
}

/// Pre-download (warm) the art cache for a batch of card names so the deckbuilder
/// doesn't trickle images in. Cache hits are instant; misses fetch through the
/// same throttle as `/art`. The client sends modest batches and shows progress.
async fn post_art_warm(State(app): State<AppState>, body: String) -> Response {
    let req: WarmRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, err(format!("invalid /art/warm body: {e}"))).into_response()
        }
    };
    let mut ok = 0usize;
    let requested = req.names.len().min(200); // bound work per request
    for name in req.names.into_iter().take(200) {
        if app.art.fetch(&name).await.is_some() {
            ok += 1;
        }
    }
    Json(WarmResponse { requested, ok, failed: requested - ok }).into_response()
}

/// Start (or report) a background job that downloads + caches art for EVERY card
/// in the catalog so the whole collection is available locally/offline. Runs one
/// at a time; already-cached cards are skipped instantly. Poll `/art/warm-status`
/// for progress. ~8k cards at the throttle ≈ 12–15 min the first time.
async fn post_art_warm_all(State(app): State<AppState>) -> Response {
    if app.art.warm.running.swap(true, Ordering::SeqCst) {
        return Json(app.art.warm_status()).into_response(); // already running
    }
    // The registry lives on the game worker thread — ask it for every name.
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::AllCardNames { reply }).is_err() {
        app.art.warm.running.store(false, Ordering::SeqCst);
        return worker_gone();
    }
    let names = match rx.await {
        Ok(n) => n,
        Err(_) => { app.art.warm.running.store(false, Ordering::SeqCst); return worker_gone(); }
    };
    app.art.warm.total.store(names.len(), Ordering::SeqCst);
    app.art.warm.done.store(0, Ordering::SeqCst);
    app.art.warm.failed.store(0, Ordering::SeqCst);
    let art = app.art.clone();
    tokio::spawn(async move {
        for name in names {
            if art.fetch(&name).await.is_none() {
                art.warm.failed.fetch_add(1, Ordering::Relaxed);
            }
            art.warm.done.fetch_add(1, Ordering::Relaxed);
        }
        art.warm.running.store(false, Ordering::Relaxed);
    });
    Json(app.art.warm_status()).into_response()
}

async fn get_art_warm_status(State(app): State<AppState>) -> Response {
    Json(app.art.warm_status()).into_response()
}

/// Cached/proxied card art (see [`ArtCache`]). Long-lived cache headers so the
/// browser also caches it; a miss/failure 404s and the client shows its fallback.
async fn get_art(State(app): State<AppState>, Query(q): Query<ArtRequest>) -> Response {
    match app.art.fetch(&q.name).await {
        Some(bytes) => (
            [
                (axum::http::header::CONTENT_TYPE, "image/jpeg"),
                (axum::http::header::CACHE_CONTROL, "public, max-age=2592000"),
            ],
            bytes,
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "art unavailable").into_response(),
    }
}

async fn post_autopass(State(app): State<AppState>, body: String) -> Response {
    let req: AutoPassRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, err(format!("invalid /autopass body: {e}"))).into_response()
        }
    };
    let level = match req.level.as_str() {
        "none" => AutoPass::None,
        "stops" => AutoPass::Stops,
        "full" => AutoPass::Full,
        other => {
            return (StatusCode::BAD_REQUEST, err(format!("unknown auto-pass level: {other}"))).into_response()
        }
    };
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::SetAutoPass { level, reply }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(resp) => Json(resp).into_response(),
        Err(_) => worker_gone(),
    }
}

async fn post_activate(State(app): State<AppState>, body: String) -> Response {
    let req: AutoTapRequest = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            return (StatusCode::BAD_REQUEST, err(format!("invalid /activate body: {e}"))).into_response()
        }
    };
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::Activate { source: req.object_id, reply }).is_err() {
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

async fn get_sample_deck(State(app): State<AppState>) -> Response {
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::SampleDeck { reply }).is_err() {
        return worker_gone();
    }
    match rx.await {
        Ok(ids) => Json(ids).into_response(),
        Err(_) => worker_gone(),
    }
}

async fn post_new(State(app): State<AppState>, body: String) -> Response {
    // Lenient: empty body is allowed (→ default → time-based seed, sample deck).
    let req: NewRequest = serde_json::from_str(&body).unwrap_or_default();
    let (reply, rx) = oneshot::channel();
    if app.tx.send(Command::New {
        seed: req.seed, deck: req.deck, opponent: req.opponent, reply,
    }).is_err() {
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
        .route("/theme.css", get(theme_css))
        .route("/app.js", get(app_js))
        .route("/formats", get(get_formats))
        .route("/import", post(post_import))
        .route("/legality", post(post_legality))
        .route("/sample-deck", get(get_sample_deck))
        .route("/glossary", get(get_glossary))
        .route("/state", get(get_state))
        .route("/suggest", get(get_suggest))
        .route("/action", post(post_action))
        .route("/combat", post(post_combat))
        .route("/autotap", post(post_autotap))
        .route("/activate", post(post_activate))
        .route("/autopass", post(post_autopass))
        .route("/bottom", post(post_bottom))
        .route("/search", post(post_search))
        .route("/new", post(post_new))
        .route("/art", get(get_art))
        .route("/art/warm", post(post_art_warm))
        .route("/art/warm-all", post(post_art_warm_all))
        .route("/art/warm-status", get(get_art_warm_status))
        .with_state(AppState { tx, art: Arc::new(ArtCache::new()) });

    // Port is overridable via PORT for convenience; defaults to 8080.
    let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {addr}: {e}"));
    println!("arcana-web listening on http://{addr}  (open it in a browser)");
    axum::serve(listener, app).await.expect("server error");
}
