//! Server-side match registry + lobby for networked 2-player duels.
//!
//! The solo vs-AI game lives in its own global [`GameCore`] (untouched by this
//! module); this registry holds the *networked* games keyed by a short join
//! code. A host creates a lobby (taking seat 0); a guest joins by code (seat 1),
//! at which point the two-human [`GameCore`] is built and the match goes
//! [`MatchStatus::Active`]. Every play call carries `(code, seat, token)` and is
//! authenticated before touching the game, so one client can't act as the other.
//!
//! CONCURRENCY: each Active match runs its [`GameCore`] on its OWN OS thread
//! (`run_match`), so heavy work in one game (e.g. a `suggest` rollout) never
//! blocks another match or the solo game. `Matches` is the Send+Sync coordinator
//! — it holds only per-match command SENDERS + lobby metadata, lives in the
//! axum `AppState` behind a `Mutex`, and authenticates/routes without touching
//! any game. Async handlers get a [`MatchSender`] (lock released first) and
//! await the per-match thread, so no lock is ever held across an await. This
//! sidesteps `GameCore: !Send`: the core is built from `Send` decklists INSIDE
//! its thread and never crosses a thread boundary.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

use arcana_core::actions::Action;

use arcana_core::objects::ObjectId;
use arcana_core::registry::CardRegistry;
use arcana_core::types::{CardId, PlayerId};

use crate::{
    CombatSubmission, DeckIdentity, GameCore, PlayerProfile, StateResponse, Suggestion,
};

/// Code alphabet: uppercase, no visually-ambiguous glyphs (O/0, I/1/L).
const CODE_ALPHABET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";
const CODE_LEN: usize = 4;

/// Lifecycle of a networked match.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchStatus {
    /// Host has created it; waiting for a guest to join.
    Lobby,
    /// Both seats filled; the game is live.
    Active,
    /// The game has ended.
    Over,
}

/// One seat in a networked match (server-private: holds the deck + secret token).
struct SeatSlot {
    filled: bool,
    token: String,
    profile: PlayerProfile,
    identity: DeckIdentity,
    deck: Vec<CardId>,
}

impl SeatSlot {
    fn empty() -> Self {
        Self {
            filled: false,
            token: String::new(),
            profile: PlayerProfile { name: String::new() },
            identity: DeckIdentity::default(),
            deck: Vec::new(),
        }
    }
}

/// Serializable snapshot of an in-progress match — everything needed to
/// reconstruct it after a server restart: seat metadata (tokens/decks) plus the
/// replayable action transcript. Written by the match thread after each action
/// when persistence is enabled (`MATCH_STATE_DIR`).
#[derive(Clone, Serialize, Deserialize)]
struct PersistedSeat {
    token: String,
    name: String,
    identity: DeckIdentity,
    deck: Vec<CardId>,
}

#[derive(Clone, Serialize, Deserialize)]
struct PersistedMatch {
    code: String,
    seed: u64,
    seats: Vec<PersistedSeat>,
    actions: Vec<Action>,
}

/// Where + what a match thread writes after each action (the `actions` are
/// refreshed from the live game before each write). `None` when persistence is off.
type PersistWriter = (PathBuf, PersistedMatch);

/// A command to a per-match game thread. The acting `seat` is already
/// authenticated by the coordinator; each carries a oneshot reply. Private — the
/// async layer talks to a match only through [`MatchSender`].
enum MatchCmd {
    State { seat: PlayerId, reply: oneshot::Sender<Result<StateResponse, String>> },
    Action { seat: PlayerId, index: usize, reply: oneshot::Sender<Result<StateResponse, String>> },
    Combat { seat: PlayerId, sub: CombatSubmission, reply: oneshot::Sender<Result<StateResponse, String>> },
    AutoTap { seat: PlayerId, target: ObjectId, reply: oneshot::Sender<Result<StateResponse, String>> },
    Activate { seat: PlayerId, source: ObjectId, reply: oneshot::Sender<Result<StateResponse, String>> },
    Bottom { seat: PlayerId, ids: Vec<ObjectId>, reply: oneshot::Sender<Result<StateResponse, String>> },
    SetAutoPass { seat: PlayerId, level: arcana_ai::session::AutoPass, reply: oneshot::Sender<Result<StateResponse, String>> },
    Suggest { seat: PlayerId, deep: bool, reply: oneshot::Sender<Vec<Suggestion>> },
}

/// Drives ONE match's [`GameCore`] on its own thread. Exits when the sender is
/// dropped (the match left / pruned), releasing the `GameCore`. Heavy work here
/// (suggest rollouts) is isolated to this thread.
fn run_match(mut core: GameCore, mut rx: mpsc::UnboundedReceiver<MatchCmd>, persist: Option<PersistWriter>) {
    while let Some(cmd) = rx.blocking_recv() {
        // A command that can change the game state → re-persist the transcript.
        let mutating = matches!(cmd,
            MatchCmd::Action { .. } | MatchCmd::Combat { .. } | MatchCmd::AutoTap { .. }
            | MatchCmd::Activate { .. } | MatchCmd::Bottom { .. });
        match cmd {
            MatchCmd::State { seat, reply } => {
                let _ = reply.send(Ok(core.snapshot_for(seat)));
            }
            MatchCmd::Action { seat, index, reply } => {
                let _ = reply.send(core.apply_index_for(seat, index).map_err(|e| e.to_string()));
            }
            MatchCmd::Combat { seat, sub, reply } => {
                let _ = reply.send(core.apply_combat_for(seat, sub).map_err(|e| e.to_string()));
            }
            MatchCmd::AutoTap { seat, target, reply } => {
                let _ = reply.send(core.auto_tap_and_cast_for(seat, target).map_err(|e| e.to_string()));
            }
            MatchCmd::Activate { seat, source, reply } => {
                let _ = reply.send(core.auto_tap_and_activate_for(seat, source).map_err(|e| e.to_string()));
            }
            MatchCmd::Bottom { seat, ids, reply } => {
                let _ = reply.send(core.bottom_cards_for(seat, ids).map_err(|e| e.to_string()));
            }
            MatchCmd::SetAutoPass { seat, level, reply } => {
                core.set_auto_pass(level);
                let _ = reply.send(Ok(core.snapshot_for(seat)));
            }
            MatchCmd::Suggest { seat, deep, reply } => {
                let _ = reply.send(core.suggest_for(seat, deep));
            }
        }
        if mutating {
            if let Some((path, meta)) = persist.as_ref() {
                let mut pm = meta.clone();
                pm.actions = core.record().actions.clone();
                if let Ok(json) = serde_json::to_string(&pm) {
                    let _ = std::fs::write(path, json);
                }
            }
        }
    }
}

/// A handle to one match's game thread, returned by [`Matches::route`] after
/// authentication. Encapsulates the oneshot round-trip so callers never see
/// [`MatchCmd`] and never hold the registry lock across the await. A send/recv
/// failure means the match thread is gone (left / reaped) → surfaced as an error.
pub struct MatchSender(mpsc::UnboundedSender<MatchCmd>);

impl MatchSender {
    async fn ask(
        &self,
        make: impl FnOnce(oneshot::Sender<Result<StateResponse, String>>) -> MatchCmd,
    ) -> Result<StateResponse, String> {
        let (reply, rx) = oneshot::channel();
        self.0.send(make(reply)).map_err(|_| "the match has ended".to_string())?;
        match rx.await {
            Ok(inner) => inner,
            Err(_) => Err("the match has ended".to_string()),
        }
    }

    pub async fn state(&self, seat: PlayerId) -> Result<StateResponse, String> {
        self.ask(|reply| MatchCmd::State { seat, reply }).await
    }
    pub async fn action(&self, seat: PlayerId, index: usize) -> Result<StateResponse, String> {
        self.ask(|reply| MatchCmd::Action { seat, index, reply }).await
    }
    pub async fn combat(&self, seat: PlayerId, sub: CombatSubmission) -> Result<StateResponse, String> {
        self.ask(|reply| MatchCmd::Combat { seat, sub, reply }).await
    }
    pub async fn auto_tap(&self, seat: PlayerId, target: ObjectId) -> Result<StateResponse, String> {
        self.ask(|reply| MatchCmd::AutoTap { seat, target, reply }).await
    }
    pub async fn activate(&self, seat: PlayerId, source: ObjectId) -> Result<StateResponse, String> {
        self.ask(|reply| MatchCmd::Activate { seat, source, reply }).await
    }
    pub async fn bottom(&self, seat: PlayerId, ids: Vec<ObjectId>) -> Result<StateResponse, String> {
        self.ask(|reply| MatchCmd::Bottom { seat, ids, reply }).await
    }
    pub async fn set_auto_pass(
        &self, seat: PlayerId, level: arcana_ai::session::AutoPass,
    ) -> Result<StateResponse, String> {
        self.ask(|reply| MatchCmd::SetAutoPass { seat, level, reply }).await
    }
    pub async fn suggest(&self, seat: PlayerId, deep: bool) -> Vec<Suggestion> {
        let (reply, rx) = oneshot::channel();
        if self.0.send(MatchCmd::Suggest { seat, deep, reply }).is_err() {
            return Vec::new();
        }
        rx.await.unwrap_or_default()
    }
}

/// One networked match: its lobby slots and, once a guest joins, a SENDER to the
/// match's game thread (the [`GameCore`] lives on that thread, not here).
struct NetMatch {
    code: String,
    status: MatchStatus,
    seed: u64,
    /// Monotonic creation order — recency key for [`Matches::prune`] (no clock).
    seq: u64,
    seats: [SeatSlot; 2],
    /// Command channel to this match's game thread. `None` until a guest joins
    /// (the thread + `GameCore` are created then). Dropping it stops the thread.
    tx: Option<mpsc::UnboundedSender<MatchCmd>>,
    /// Last contact time per seat (monotonic). Bumped on any authenticated
    /// command ([`Matches::route`]) and by the open WebSocket's keepalive
    /// ([`Matches::touch`]). A filled seat that goes silent past the reap timeout
    /// is treated as vanished, and [`Matches::reap_stale`] drops the match so the
    /// present player isn't frozen forever.
    last_seen: [Instant; 2],
}

/// The credentials a client keeps after creating/joining: which match, which
/// seat, and the secret token that authorizes acting as that seat.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SeatCredentials {
    pub code: String,
    pub seat: PlayerId,
    pub token: String,
}

/// Public lobby snapshot (no decks, no tokens) — what a client polls to see
/// whether the opponent has joined and who's across the table.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LobbyInfo {
    pub code: String,
    pub status: MatchStatus,
    pub seats: Vec<SeatInfo>,
}

/// Public per-seat info for [`LobbyInfo`] (presentation only).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeatInfo {
    pub filled: bool,
    pub name: String,
    pub identity: DeckIdentity,
}

/// Cap on retained matches before idle lobbies get evicted (a local-tool
/// backstop against the registry growing without bound).
const MAX_MATCHES: usize = 64;

/// The registry of networked matches. One per server, owned by the game worker.
pub struct Matches {
    reg: &'static CardRegistry,
    by_code: HashMap<String, NetMatch>,
    /// splitmix64 state for codes / tokens / seeds (no `rand` dependency).
    rng: u64,
    /// Monotonic match-creation counter (recency for [`prune`](Self::prune)).
    next_seq: u64,
    /// When set, Active matches are persisted here (one `<code>.json` transcript
    /// each) and reloaded on startup, so a restart doesn't drop live games.
    state_dir: Option<PathBuf>,
}

impl Matches {
    /// `seed` initializes the code/token RNG (the server passes a time seed).
    /// `state_dir`, when set, enables on-disk match persistence + startup resume.
    pub fn new(reg: &'static CardRegistry, seed: u64, state_dir: Option<PathBuf>) -> Self {
        let mut m = Self { reg, by_code: HashMap::new(), rng: seed | 1, next_seq: 0, state_dir };
        m.load_persisted();
        m
    }

    /// Path of a match's persistence file (`<state_dir>/<code>.json`), if enabled.
    fn persist_path(&self, code: &str) -> Option<PathBuf> {
        self.state_dir.as_ref().map(|d| d.join(format!("{code}.json")))
    }

    /// Build the per-match transcript writer handed to its game thread (or `None`
    /// when persistence is off).
    fn persist_writer(&self, code: &str, seed: u64, seats: &[SeatSlot; 2]) -> Option<PersistWriter> {
        let path = self.persist_path(code)?;
        let pseats = seats.iter().map(|s| PersistedSeat {
            token: s.token.clone(),
            name: s.profile.name.clone(),
            identity: s.identity.clone(),
            deck: s.deck.clone(),
        }).collect();
        Some((path, PersistedMatch { code: code.to_string(), seed, seats: pseats, actions: Vec::new() }))
    }

    /// Delete a match's persistence file (on leave / reap / prune).
    fn del_persist(&self, code: &str) {
        if let Some(p) = self.persist_path(code) {
            let _ = std::fs::remove_file(p);
        }
    }

    /// Spawn a match's game thread. `actions` empty → a fresh game; non-empty →
    /// resume by replaying the transcript. The `GameCore` (`!Send` via bots, though
    /// two-human has none) is BUILT INSIDE the thread from `Send` inputs.
    fn spawn_match(
        reg: &'static CardRegistry, seed: u64, deck0: Vec<CardId>, deck1: Vec<CardId>,
        actions: Vec<Action>, code: &str, persist: Option<PersistWriter>,
    ) -> mpsc::UnboundedSender<MatchCmd> {
        let (tx, rx) = mpsc::unbounded_channel::<MatchCmd>();
        std::thread::Builder::new()
            .name(format!("match-{code}"))
            .spawn(move || {
                let core = if actions.is_empty() {
                    GameCore::new_two_human(reg, seed, deck0, deck1)
                } else {
                    GameCore::resume_two_human(reg, seed, deck0, deck1, &actions)
                };
                run_match(core, rx, persist);
            })
            .expect("spawn match thread");
        tx
    }

    /// Reload persisted in-progress matches from `state_dir` at startup.
    fn load_persisted(&mut self) {
        let Some(dir) = self.state_dir.clone() else { return; };
        let _ = std::fs::create_dir_all(&dir);
        let Ok(entries) = std::fs::read_dir(&dir) else { return; };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|x| x.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(pm) = serde_json::from_str::<PersistedMatch>(&text) else { continue };
            self.restore(pm);
        }
    }

    /// Reconstruct one match from its persisted transcript (spawns a resuming
    /// game thread and re-registers the Active match).
    fn restore(&mut self, pm: PersistedMatch) {
        if pm.seats.len() != 2 {
            return;
        }
        let seat_of = |ps: &PersistedSeat| SeatSlot {
            filled: true,
            token: ps.token.clone(),
            profile: PlayerProfile { name: ps.name.clone() },
            identity: ps.identity.clone(),
            deck: ps.deck.clone(),
        };
        let seats = [seat_of(&pm.seats[0]), seat_of(&pm.seats[1])];
        let (deck0, deck1) = (seats[0].deck.clone(), seats[1].deck.clone());
        let persist = self.persist_writer(&pm.code, pm.seed, &seats);
        let tx = Self::spawn_match(self.reg, pm.seed, deck0, deck1, pm.actions, &pm.code, persist);
        let seq = self.next_seq;
        self.next_seq += 1;
        self.by_code.insert(pm.code.clone(), NetMatch {
            code: pm.code,
            status: MatchStatus::Active,
            seed: pm.seed,
            seq,
            seats,
            tx: Some(tx),
            last_seen: [Instant::now(); 2],
        });
    }

    /// Reclaim space: drop finished (`Over`) matches outright, and if still over
    /// [`MAX_MATCHES`] evict the oldest never-joined lobbies. Active games are
    /// never evicted. Called before opening a new match. Deterministic (recency
    /// by creation `seq`, no wall clock).
    fn prune(&mut self) {
        let over: Vec<String> = self.by_code.iter()
            .filter(|(_, m)| m.status == MatchStatus::Over)
            .map(|(c, _)| c.clone())
            .collect();
        self.by_code.retain(|_, m| m.status != MatchStatus::Over);
        for code in &over {
            self.del_persist(code);
        }
        if self.by_code.len() <= MAX_MATCHES {
            return;
        }
        let mut lobbies: Vec<(u64, String)> = self.by_code.iter()
            .filter(|(_, m)| m.status == MatchStatus::Lobby)
            .map(|(c, m)| (m.seq, c.clone()))
            .collect();
        lobbies.sort_by_key(|(seq, _)| *seq);
        let mut excess = self.by_code.len().saturating_sub(MAX_MATCHES);
        for (_, code) in lobbies {
            if excess == 0 { break; }
            self.by_code.remove(&code);
            excess -= 1;
        }
    }

    /// Leave (or cancel) a match: the requesting seat must own its token. The
    /// whole match is removed — a host cancels an unjoined lobby, or a player
    /// bows out of a live game (the opponent's next poll then reports the match
    /// is gone).
    pub fn leave(&mut self, code: &str, seat: PlayerId, token: &str) -> Result<(), String> {
        let m = self.by_code.get(code)
            .ok_or_else(|| "no match with that code".to_string())?;
        let ok = m.seats.get(seat as usize)
            .is_some_and(|s| s.filled && s.token == token);
        if !ok {
            return Err("not authorized for this seat".to_string());
        }
        self.by_code.remove(code);
        self.del_persist(code);
        Ok(())
    }

    fn next_rand(&mut self) -> u64 {
        // splitmix64.
        self.rng = self.rng.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.rng;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn token(&mut self) -> String {
        format!("{:016x}", self.next_rand())
    }

    fn fresh_code(&mut self) -> String {
        loop {
            let mut n = self.next_rand();
            let mut code = String::with_capacity(CODE_LEN);
            for _ in 0..CODE_LEN {
                code.push(CODE_ALPHABET[(n % CODE_ALPHABET.len() as u64) as usize] as char);
                n /= CODE_ALPHABET.len() as u64;
            }
            if !self.by_code.contains_key(&code) {
                return code;
            }
        }
    }

    /// Host a new match: take seat 0 with `deck`, open a lobby, and return the
    /// host's seat credentials (the code to share + the secret token).
    pub fn create(
        &mut self, profile: PlayerProfile, identity: DeckIdentity, deck: Vec<CardId>,
    ) -> Result<SeatCredentials, String> {
        crate::validate_deck(self.reg, &deck)?;
        self.prune();
        let code = self.fresh_code();
        let token = self.token();
        let seed = self.next_rand();
        let seq = self.next_seq;
        self.next_seq += 1;
        let host = SeatSlot {
            filled: true, token: token.clone(), profile, identity, deck,
        };
        self.by_code.insert(code.clone(), NetMatch {
            code: code.clone(),
            status: MatchStatus::Lobby,
            seed,
            seq,
            seats: [host, SeatSlot::empty()],
            tx: None,
            last_seen: [Instant::now(); 2],
        });
        Ok(SeatCredentials { code, seat: 0, token })
    }

    /// Join an open lobby by `code`, taking seat 1 with `deck`. Builds the
    /// two-human game (both decks now known) and flips the match to Active.
    pub fn join(
        &mut self, code: &str, profile: PlayerProfile, identity: DeckIdentity, deck: Vec<CardId>,
    ) -> Result<SeatCredentials, String> {
        crate::validate_deck(self.reg, &deck)?;
        let token = self.token();
        let m = self.by_code.get_mut(code)
            .ok_or_else(|| "no match with that code".to_string())?;
        if m.status != MatchStatus::Lobby {
            return Err("that match has already started".to_string());
        }
        if m.seats[1].filled {
            return Err("that match is full".to_string());
        }
        m.seats[1] = SeatSlot {
            filled: true, token: token.clone(), profile, identity, deck,
        };
        // Gather (immutable) then spawn the game thread — a fresh two-human game.
        let (reg, seed) = (self.reg, self.by_code[code].seed);
        let m_ref = &self.by_code[code];
        let (deck0, deck1) = (m_ref.seats[0].deck.clone(), m_ref.seats[1].deck.clone());
        let persist = self.persist_writer(code, seed, &m_ref.seats);
        let tx = Self::spawn_match(reg, seed, deck0, deck1, Vec::new(), code, persist);
        let m = self.by_code.get_mut(code).unwrap();
        m.tx = Some(tx);
        m.status = MatchStatus::Active;
        m.last_seen = [Instant::now(); 2]; // both present at kickoff
        Ok(SeatCredentials { code: code.to_string(), seat: 1, token })
    }

    /// Public lobby info for `code` (presentation only — no decks/tokens), or
    /// `None` if there's no such match.
    pub fn info(&self, code: &str) -> Option<LobbyInfo> {
        let m = self.by_code.get(code)?;
        Some(LobbyInfo {
            code: m.code.clone(),
            status: m.status,
            seats: m.seats.iter().map(|s| SeatInfo {
                filled: s.filled,
                name: s.profile.name.clone(),
                identity: s.identity.clone(),
            }).collect(),
        })
    }

    /// Authenticate `(code, seat, token)` and return a [`MatchSender`] to the
    /// match's game thread. The caller (an async handler) sends/awaits AFTER
    /// releasing the registry lock — no lock is held across the await.
    pub fn route(&mut self, code: &str, seat: PlayerId, token: &str)
        -> Result<MatchSender, String>
    {
        let m = self.by_code.get_mut(code)
            .ok_or_else(|| "no match with that code".to_string())?;
        let ok = m.seats.get(seat as usize)
            .is_some_and(|s| s.filled && s.token == token);
        if !ok {
            return Err("not authorized for this seat".to_string());
        }
        if let Some(t) = m.last_seen.get_mut(seat as usize) {
            *t = Instant::now(); // this seat is alive
        }
        m.tx.clone()
            .map(MatchSender)
            .ok_or_else(|| "that match has not started yet".to_string())
    }

    /// Keepalive: mark `seat` alive (the open WebSocket calls this on a timer so a
    /// connected-but-quiet client isn't reaped between game actions). Unauthenticated
    /// — the socket was already token-checked on connect.
    pub fn touch(&mut self, code: &str, seat: PlayerId) {
        if let Some(m) = self.by_code.get_mut(code) {
            if let Some(t) = m.last_seen.get_mut(seat as usize) {
                *t = Instant::now();
            }
        }
    }

    /// Drop every Active match with a filled seat that hasn't been seen within
    /// `timeout` (a vanished player), returning the removed codes so the caller can
    /// notify the surviving client (its next fetch 404s → an `ended` notice).
    /// Dropping the entry drops the match's command sender, ending its game thread.
    pub fn reap_stale(&mut self, timeout: Duration) -> Vec<String> {
        let now = Instant::now();
        let stale: Vec<String> = self.by_code.iter()
            .filter(|(_, m)| m.status == MatchStatus::Active)
            .filter(|(_, m)| m.seats.iter().enumerate().any(|(i, s)|
                s.filled && now.duration_since(m.last_seen[i]) > timeout))
            .map(|(c, _)| c.clone())
            .collect();
        for code in &stale {
            self.by_code.remove(code);
            self.del_persist(code);
        }
        stale
    }

    /// Mark a match finished (handlers call this when a pushed view shows
    /// game_over) so [`prune`](Self::prune) can reclaim it.
    pub fn mark_over(&mut self, code: &str) {
        if let Some(m) = self.by_code.get_mut(code) {
            m.status = MatchStatus::Over;
        }
    }
}

/// Synchronous round-trips to a match thread, for unit tests (no async runtime).
#[cfg(test)]
impl Matches {
    fn snapshot(&mut self, code: &str, seat: PlayerId, token: &str) -> Result<StateResponse, String> {
        let s = self.route(code, seat, token)?;
        let (reply, rx) = oneshot::channel();
        s.0.send(MatchCmd::State { seat, reply }).map_err(|_| "match ended".to_string())?;
        rx.blocking_recv().map_err(|_| "match ended".to_string())?
    }
    fn action(&mut self, code: &str, seat: PlayerId, token: &str, index: usize) -> Result<StateResponse, String> {
        let s = self.route(code, seat, token)?;
        let (reply, rx) = oneshot::channel();
        s.0.send(MatchCmd::Action { seat, index, reply }).map_err(|_| "match ended".to_string())?;
        rx.blocking_recv().map_err(|_| "match ended".to_string())?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaked_catalog() -> &'static CardRegistry {
        Box::leak(Box::new(arcana_cards::build_catalog()))
    }

    fn deck(reg: &'static CardRegistry) -> Vec<CardId> {
        arcana_cards::sample_deck(reg, crate::DECK_SEED)
    }

    fn profile(name: &str) -> PlayerProfile {
        PlayerProfile { name: name.to_string() }
    }

    /// Host → join builds an Active two-human game; each seat can poll its own
    /// perspective, and the host sees the guest appear in the lobby info.
    #[test]
    fn create_then_join_starts_a_two_human_game() {
        let reg = leaked_catalog();
        let mut m = Matches::new(reg, 12345, None);

        let host = m.create(profile("Alice"), DeckIdentity::default(), deck(reg)).unwrap();
        assert_eq!(host.seat, 0);

        // Before anyone joins it's a lobby with seat 1 empty.
        let info = m.info(&host.code).unwrap();
        assert_eq!(info.status, MatchStatus::Lobby);
        assert!(info.seats[0].filled && !info.seats[1].filled);

        let guest = m.join(&host.code, profile("Bob"), DeckIdentity::default(), deck(reg)).unwrap();
        assert_eq!(guest.seat, 1);
        assert_eq!(guest.code, host.code);
        assert_ne!(guest.token, host.token, "each seat gets its own token");

        let info = m.info(&host.code).unwrap();
        assert_eq!(info.status, MatchStatus::Active);
        assert_eq!(info.seats[1].name, "Bob");

        // Both seats can poll their own perspective off the one shared game.
        let s0 = m.snapshot(&host.code, 0, &host.token).unwrap();
        let s1 = m.snapshot(&host.code, 1, &guest.token).unwrap();
        assert_eq!(s0.view.perspective, 0);
        assert_eq!(s1.view.perspective, 1);
        // Exactly one of them is to act; the other is waiting (no actions).
        assert_ne!(s0.view.legal.is_empty(), s1.view.legal.is_empty(),
            "exactly one seat is on the clock");
    }

    /// The reaper drops Active matches whose player vanished (stale past the
    /// timeout), never touches lobbies, keeps fresh/just-touched matches, and
    /// returns the removed codes so the survivor can be notified.
    #[test]
    fn reap_stale_drops_vanished_matches() {
        use std::time::Duration;
        let reg = leaked_catalog();
        let mut m = Matches::new(reg, 77, None);

        // A lobby is never reaped, even at zero timeout.
        let lob = m.create(profile("Solo"), DeckIdentity::default(), deck(reg)).unwrap();
        std::thread::sleep(Duration::from_millis(2));
        assert!(m.reap_stale(Duration::ZERO).is_empty(), "lobbies are never reaped");
        assert!(m.info(&lob.code).is_some());

        // A fresh Active match under a generous timeout is kept.
        let host = m.create(profile("Alice"), DeckIdentity::default(), deck(reg)).unwrap();
        m.join(&host.code, profile("Bob"), DeckIdentity::default(), deck(reg)).unwrap();
        assert!(m.reap_stale(Duration::from_secs(3600)).is_empty(), "fresh match kept");
        assert!(m.info(&host.code).is_some());

        // After a beat, a zero timeout sees both seats as vanished → reaped, and
        // the removed code is returned.
        std::thread::sleep(Duration::from_millis(2));
        let reaped = m.reap_stale(Duration::ZERO);
        assert_eq!(reaped, vec![host.code.clone()]);
        assert!(m.info(&host.code).is_none(), "reaped match is gone");

        // touch() resets liveness: a just-touched match survives a 1s-timeout reap.
        let h2 = m.create(profile("Cara"), DeckIdentity::default(), deck(reg)).unwrap();
        m.join(&h2.code, profile("Dan"), DeckIdentity::default(), deck(reg)).unwrap();
        std::thread::sleep(Duration::from_millis(2));
        m.touch(&h2.code, 0);
        m.touch(&h2.code, 1);
        assert!(m.reap_stale(Duration::from_secs(1)).is_empty(), "just-touched match kept");
    }

    /// A persisted match transcript on disk is reloaded into an Active, playable
    /// match when a new registry starts (restart recovery). Deletion on leave is
    /// also exercised.
    #[test]
    fn persisted_match_is_restored_on_startup() {
        let reg = leaked_catalog();
        let dir = std::env::temp_dir().join(format!("arcana-mm-{}-restore", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // Hand-write a fresh (empty-transcript) persisted match.
        let pm = PersistedMatch {
            code: "RSTR".into(),
            seed: 42,
            seats: vec![
                PersistedSeat { token: "tok0".into(), name: "Ada".into(),
                    identity: DeckIdentity::default(), deck: deck(reg) },
                PersistedSeat { token: "tok1".into(), name: "Ben".into(),
                    identity: DeckIdentity::default(), deck: deck(reg) },
            ],
            actions: Vec::new(),
        };
        std::fs::write(dir.join("RSTR.json"), serde_json::to_string(&pm).unwrap()).unwrap();

        // A new registry over that dir loads the match.
        let mut m = Matches::new(reg, 1, Some(dir.clone()));
        let info = m.info("RSTR").expect("restored match is present");
        assert_eq!(info.status, MatchStatus::Active);
        assert_eq!(info.seats[0].name, "Ada");
        assert_eq!(info.seats[1].name, "Ben");
        // Playable with the persisted tokens; wrong token is refused.
        let s = m.snapshot("RSTR", 0, "tok0").expect("restored game answers seat 0");
        assert_eq!(s.view.perspective, 0);
        assert!(m.snapshot("RSTR", 0, "wrong").is_err(), "token still enforced");

        // Leaving deletes the persisted file.
        m.leave("RSTR", 0, "tok0").unwrap();
        assert!(!dir.join("RSTR.json").exists(), "leave removes the persisted file");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Bad join codes, a full match, and a second start are all rejected.
    #[test]
    fn join_failures_are_reported() {
        let reg = leaked_catalog();
        let mut m = Matches::new(reg, 999, None);
        assert!(m.join("ZZZZ", profile("x"), DeckIdentity::default(), deck(reg)).is_err());

        let host = m.create(profile("Alice"), DeckIdentity::default(), deck(reg)).unwrap();
        m.join(&host.code, profile("Bob"), DeckIdentity::default(), deck(reg)).unwrap();
        // Now full / already started.
        let third = m.join(&host.code, profile("Eve"), DeckIdentity::default(), deck(reg));
        assert!(third.is_err(), "a started match can't be joined again");

        // An empty deck is refused on both create and join.
        assert!(m.create(profile("x"), DeckIdentity::default(), Vec::new()).is_err());

        // W-2 regression: an unregistered card id is refused at create AND join
        // (previously it panicked the spawned match thread — silent match death).
        let mut poisoned = deck(reg);
        poisoned[0] = u32::MAX;
        let err = m.create(profile("x"), DeckIdentity::default(), poisoned.clone()).unwrap_err();
        assert!(err.contains("unknown card id"), "got: {err}");
        let host2 = m.create(profile("Alice"), DeckIdentity::default(), deck(reg)).unwrap();
        let err = m.join(&host2.code, profile("Mallory"), DeckIdentity::default(), poisoned)
            .unwrap_err();
        assert!(err.contains("unknown card id"), "got: {err}");
        // A lobby the poisoned join bounced off is still joinable.
        assert!(m.join(&host2.code, profile("Bob"), DeckIdentity::default(), deck(reg)).is_ok());

        // Oversized decks are capped.
        let card = deck(reg)[0];
        let err = m.create(profile("x"), DeckIdentity::default(),
            vec![card; crate::MAX_DECK_SIZE + 1]).unwrap_err();
        assert!(err.contains("at most"), "got: {err}");
    }

    /// A client must present the right token for its seat, and can't act as the
    /// other seat or out of turn.
    #[test]
    fn token_and_turn_are_enforced() {
        let reg = leaked_catalog();
        let mut m = Matches::new(reg, 7, None);
        let host = m.create(profile("Alice"), DeckIdentity::default(), deck(reg)).unwrap();
        let guest = m.join(&host.code, profile("Bob"), DeckIdentity::default(), deck(reg)).unwrap();

        // Wrong token for seat 0 → unauthorized.
        assert!(m.snapshot(&host.code, 0, "deadbeef").is_err());
        // Using the guest's token to act as seat 0 → unauthorized.
        assert!(m.action(&host.code, 0, &guest.token, 0).is_err());

        // Find who's to act and confirm the other seat is refused (out of turn).
        let s0 = m.snapshot(&host.code, 0, &host.token).unwrap();
        let (actor, actor_tok, waiter, waiter_tok) = if s0.view.legal.is_empty() {
            (1, &guest.token, 0, &host.token)
        } else {
            (0, &host.token, 1, &guest.token)
        };
        assert!(m.action(&host.code, waiter, waiter_tok, 0).is_err(),
            "the off-turn seat can't act");
        assert!(m.action(&host.code, actor, actor_tok, 0).is_ok(),
            "the on-turn seat can act");
    }

    /// Leaving/cancelling removes the match and requires the seat's own token.
    #[test]
    fn leave_removes_the_match_and_checks_token() {
        let reg = leaked_catalog();
        let mut m = Matches::new(reg, 1, None);
        let host = m.create(profile("Alice"), DeckIdentity::default(), deck(reg)).unwrap();
        assert!(m.leave(&host.code, 0, "wrongtoken").is_err(), "wrong token can't cancel");
        assert!(m.info(&host.code).is_some(), "still there after a bad cancel");
        assert!(m.leave(&host.code, 0, &host.token).is_ok(), "host cancels its lobby");
        assert!(m.info(&host.code).is_none(), "cancelled match is gone");
    }

    /// The registry is bounded: spamming never-joined lobbies prunes the oldest.
    #[test]
    fn idle_lobbies_are_pruned_to_a_bound() {
        let reg = leaked_catalog();
        let mut m = Matches::new(reg, 2, None);
        let d = deck(reg);
        for _ in 0..200 {
            m.create(profile("A"), DeckIdentity::default(), d.clone()).unwrap();
        }
        assert!(m.by_code.len() <= MAX_MATCHES + 1,
            "idle lobbies pruned to a bound, got {}", m.by_code.len());
    }
}
