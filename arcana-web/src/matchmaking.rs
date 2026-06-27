//! Server-side match registry + lobby for networked 2-player duels.
//!
//! The solo vs-AI game lives in its own global [`GameCore`] (untouched by this
//! module); this registry holds the *networked* games keyed by a short join
//! code. A host creates a lobby (taking seat 0); a guest joins by code (seat 1),
//! at which point the two-human [`GameCore`] is built and the match goes
//! [`MatchStatus::Active`]. Every play call carries `(code, seat, token)` and is
//! authenticated before touching the game, so one client can't act as the other.
//!
//! The HTTP layer (and the worker thread that owns one `Matches`) is wired in a
//! later phase; this module is the pure, unit-tested model underneath it.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

/// One networked match: its lobby slots and, once both seats are filled, the
/// shared two-human game.
struct NetMatch {
    code: String,
    status: MatchStatus,
    seed: u64,
    /// Monotonic creation order — recency key for [`Matches::prune`] (no clock).
    seq: u64,
    seats: [SeatSlot; 2],
    core: Option<GameCore>,
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
}

impl Matches {
    /// `seed` initializes the code/token RNG (the server passes a time seed).
    pub fn new(reg: &'static CardRegistry, seed: u64) -> Self {
        Self { reg, by_code: HashMap::new(), rng: seed | 1, next_seq: 0 }
    }

    /// Reclaim space: drop finished (`Over`) matches outright, and if still over
    /// [`MAX_MATCHES`] evict the oldest never-joined lobbies. Active games are
    /// never evicted. Called before opening a new match. Deterministic (recency
    /// by creation `seq`, no wall clock).
    fn prune(&mut self) {
        self.by_code.retain(|_, m| m.status != MatchStatus::Over);
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
        if deck.is_empty() {
            return Err("your deck is empty".to_string());
        }
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
            core: None,
        });
        Ok(SeatCredentials { code, seat: 0, token })
    }

    /// Join an open lobby by `code`, taking seat 1 with `deck`. Builds the
    /// two-human game (both decks now known) and flips the match to Active.
    pub fn join(
        &mut self, code: &str, profile: PlayerProfile, identity: DeckIdentity, deck: Vec<CardId>,
    ) -> Result<SeatCredentials, String> {
        if deck.is_empty() {
            return Err("your deck is empty".to_string());
        }
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
        let core = GameCore::new_two_human(
            self.reg, m.seed, m.seats[0].deck.clone(), m.seats[1].deck.clone());
        m.core = Some(core);
        m.status = MatchStatus::Active;
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

    /// Authenticate `(code, seat, token)` and return the live game mutably.
    fn authed_core(&mut self, code: &str, seat: PlayerId, token: &str)
        -> Result<&mut GameCore, String>
    {
        let m = self.by_code.get_mut(code)
            .ok_or_else(|| "no match with that code".to_string())?;
        let ok = m.seats.get(seat as usize)
            .is_some_and(|s| s.filled && s.token == token);
        if !ok {
            return Err("not authorized for this seat".to_string());
        }
        m.core.as_mut().ok_or_else(|| "that match has not started yet".to_string())
    }

    /// Read-only authenticated access (for `suggest`).
    fn authed_core_ref(&self, code: &str, seat: PlayerId, token: &str) -> Option<&GameCore> {
        let m = self.by_code.get(code)?;
        let s = m.seats.get(seat as usize)?;
        if !s.filled || s.token != token {
            return None;
        }
        m.core.as_ref()
    }

    /// Project the game from `seat`'s perspective (the "waiting for opponent"
    /// view when it isn't their turn). Flips the match to `Over` on game end.
    pub fn snapshot(&mut self, code: &str, seat: PlayerId, token: &str)
        -> Result<StateResponse, String>
    {
        let core = self.authed_core(code, seat, token)?;
        let resp = core.snapshot_for(seat);
        if resp.view.game_over.is_some() {
            if let Some(m) = self.by_code.get_mut(code) {
                m.status = MatchStatus::Over;
            }
        }
        Ok(resp)
    }

    pub fn action(&mut self, code: &str, seat: PlayerId, token: &str, index: usize)
        -> Result<StateResponse, String>
    {
        self.authed_core(code, seat, token)?
            .apply_index_for(seat, index).map_err(|e| e.to_string())
    }

    pub fn combat(&mut self, code: &str, seat: PlayerId, token: &str, sub: CombatSubmission)
        -> Result<StateResponse, String>
    {
        self.authed_core(code, seat, token)?
            .apply_combat_for(seat, sub).map_err(|e| e.to_string())
    }

    pub fn auto_tap(&mut self, code: &str, seat: PlayerId, token: &str, target: arcana_core::objects::ObjectId)
        -> Result<StateResponse, String>
    {
        self.authed_core(code, seat, token)?
            .auto_tap_and_cast_for(seat, target).map_err(|e| e.to_string())
    }

    pub fn activate(&mut self, code: &str, seat: PlayerId, token: &str, source: arcana_core::objects::ObjectId)
        -> Result<StateResponse, String>
    {
        self.authed_core(code, seat, token)?
            .auto_tap_and_activate_for(seat, source).map_err(|e| e.to_string())
    }

    pub fn bottom(&mut self, code: &str, seat: PlayerId, token: &str, ids: Vec<arcana_core::objects::ObjectId>)
        -> Result<StateResponse, String>
    {
        self.authed_core(code, seat, token)?
            .bottom_cards_for(seat, ids).map_err(|e| e.to_string())
    }

    pub fn suggest(&self, code: &str, seat: PlayerId, token: &str, deep: bool) -> Vec<Suggestion> {
        self.authed_core_ref(code, seat, token)
            .map_or_else(Vec::new, |c| c.suggest_for(seat, deep))
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
        let mut m = Matches::new(reg, 12345);

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

    /// Bad join codes, a full match, and a second start are all rejected.
    #[test]
    fn join_failures_are_reported() {
        let reg = leaked_catalog();
        let mut m = Matches::new(reg, 999);
        assert!(m.join("ZZZZ", profile("x"), DeckIdentity::default(), deck(reg)).is_err());

        let host = m.create(profile("Alice"), DeckIdentity::default(), deck(reg)).unwrap();
        m.join(&host.code, profile("Bob"), DeckIdentity::default(), deck(reg)).unwrap();
        // Now full / already started.
        let third = m.join(&host.code, profile("Eve"), DeckIdentity::default(), deck(reg));
        assert!(third.is_err(), "a started match can't be joined again");

        // An empty deck is refused on both create and join.
        assert!(m.create(profile("x"), DeckIdentity::default(), Vec::new()).is_err());
    }

    /// A client must present the right token for its seat, and can't act as the
    /// other seat or out of turn.
    #[test]
    fn token_and_turn_are_enforced() {
        let reg = leaked_catalog();
        let mut m = Matches::new(reg, 7);
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
        let mut m = Matches::new(reg, 1);
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
        let mut m = Matches::new(reg, 2);
        let d = deck(reg);
        for _ in 0..200 {
            m.create(profile("A"), DeckIdentity::default(), d.clone()).unwrap();
        }
        assert!(m.by_code.len() <= MAX_MATCHES + 1,
            "idle lobbies pruned to a bound, got {}", m.by_code.len());
    }
}
