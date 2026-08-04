//! Server-side deck-store sync (Phase 2.2, W-10).
//!
//! The browser's versioned, lineage-aware deck store (`Arcana.Decks` in
//! `static/app.js`) previously lived ONLY in `localStorage` — decks didn't
//! roam between devices/browsers and a cleared profile deleted them. This
//! module gives it a per-profile server home:
//!
//! * The client keeps `localStorage` as its fast cache and offline fallback,
//!   and syncs the WHOLE store as one envelope — the server treats the store
//!   itself as an opaque (but size-capped, JSON-validated) document. Game
//!   decks still enter play exclusively through the validated `/new` +
//!   `/lobby` paths, so nothing here loosens the W-1/W-2 trust boundary.
//! * A *profile* is a client-generated UUID kept in `localStorage`
//!   (`arcana.profile`). Knowing the id grants access to that profile's decks
//!   — the LAN-tool trust model, same tier as match seat tokens. Ids are
//!   sanitized to a conservative charset before touching the filesystem.
//! * Concurrency: the envelope carries a client-incremented `rev`. A PUT with
//!   `rev` older than what's stored is refused (the caller gets the stored
//!   envelope back and adopts it) — last-write-wins with clobber protection,
//!   not a merge; documented in the client.
//! * Writes are atomic (temp + rename, the W-5 lesson); one `<profile>.json`
//!   per profile under the store directory.

use std::path::PathBuf;

/// Upper bound on a stored envelope. The store carries full `CardInfo` blobs
/// per deck entry (so any page renders without a round-trip), which is bulky
/// but bounded: even a hundred 250-card decks fit comfortably.
pub const MAX_STORE_BYTES: usize = 8 * 1024 * 1024;

/// Envelope metadata the server actually reads; everything else rides along
/// opaquely inside `body`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreRev(pub u64);

/// Per-profile deck persistence rooted at one directory. `None` = disabled
/// (the client silently stays localStorage-only).
pub struct DeckStore {
    dir: Option<PathBuf>,
}

/// Outcome of a conditional save.
pub enum SaveOutcome {
    /// Stored; the new revision is echoed back.
    Saved(StoreRev),
    /// Refused: the stored envelope is newer (or as new). Carries the stored
    /// body so the caller can hand it straight back to the client.
    Stale { stored: String, stored_rev: StoreRev },
}

impl DeckStore {
    /// `dir = None` disables persistence. The directory is created eagerly so
    /// a misconfigured path fails loudly at startup, not on first save.
    pub fn new(dir: Option<PathBuf>) -> Self {
        if let Some(d) = &dir {
            if let Err(e) = std::fs::create_dir_all(d) {
                eprintln!("[arcana-web] deck store at {} unavailable: {e}", d.display());
                return Self { dir: None };
            }
        }
        Self { dir }
    }

    /// Whether persistence is on (drives the client's sync opt-in).
    pub fn enabled(&self) -> bool {
        self.dir.is_some()
    }

    /// A profile id is client-generated and becomes a filename — accept only a
    /// conservative charset and a sane length (UUIDs and the client's fallback
    /// base-36 ids both pass).
    fn profile_path(&self, profile: &str) -> Result<PathBuf, String> {
        let ok_len = (8..=64).contains(&profile.len());
        let ok_chars = profile.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
        if !(ok_len && ok_chars) {
            return Err("invalid profile id".to_string());
        }
        let dir = self.dir.as_ref().ok_or_else(|| "deck store is disabled".to_string())?;
        Ok(dir.join(format!("{profile}.json")))
    }

    /// The stored envelope for `profile`, or `None` for a fresh profile.
    pub fn load(&self, profile: &str) -> Result<Option<String>, String> {
        let path = self.profile_path(profile)?;
        match std::fs::read_to_string(&path) {
            Ok(s) => Ok(Some(s)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(format!("couldn't read the deck store: {e}")),
        }
    }

    /// Validate + conditionally store `body` (the full envelope JSON) for
    /// `profile`. Refuses bodies that are oversized, non-JSON, or whose `rev`
    /// isn't newer than the stored one.
    pub fn save(&self, profile: &str, body: &str) -> Result<SaveOutcome, String> {
        let path = self.profile_path(profile)?;
        if body.len() > MAX_STORE_BYTES {
            return Err(format!(
                "deck store too large ({} bytes; limit {MAX_STORE_BYTES})", body.len()));
        }
        let incoming_rev = envelope_rev(body)
            .ok_or_else(|| "deck store body must be JSON with a numeric `rev`".to_string())?;

        if let Some(stored) = self.load(profile)? {
            let stored_rev = envelope_rev(&stored).unwrap_or(StoreRev(0));
            if incoming_rev.0 <= stored_rev.0 {
                return Ok(SaveOutcome::Stale { stored, stored_rev });
            }
        }

        // Atomic write: a crash mid-write must never tear the profile's decks.
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, body)
            .and_then(|()| std::fs::rename(&tmp, &path))
            .map_err(|e| format!("couldn't persist the deck store: {e}"))?;
        Ok(SaveOutcome::Saved(incoming_rev))
    }
}

/// Pull `rev` out of the envelope without deserializing the whole store.
fn envelope_rev(body: &str) -> Option<StoreRev> {
    let v: serde_json::Value = serde_json::from_str(body).ok()?;
    v.get("rev")?.as_u64().map(StoreRev)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fresh per-test dir under the system tmpdir (same idiom as the
    /// matchmaking persistence tests — no tempdir dev-dependency).
    fn store(tag: &str) -> DeckStore {
        let dir = std::env::temp_dir()
            .join(format!("arcana-deckstore-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        DeckStore::new(Some(dir))
    }

    #[test]
    fn roundtrip_and_rev_guard() {
        let s = store("roundtrip");
        let p = "11111111-2222-3333-4444-555555555555";
        assert_eq!(s.load(p).unwrap(), None, "fresh profile is empty");

        let v1 = r#"{"rev":1,"updated":10,"store":{"version":2,"decks":{}}}"#;
        assert!(matches!(s.save(p, v1).unwrap(), SaveOutcome::Saved(StoreRev(1))));
        assert_eq!(s.load(p).unwrap().as_deref(), Some(v1));

        // A newer rev replaces; an older/equal one is refused with the stored copy.
        let v2 = r#"{"rev":2,"updated":20,"store":{"version":2,"decks":{}}}"#;
        assert!(matches!(s.save(p, v2).unwrap(), SaveOutcome::Saved(StoreRev(2))));
        match s.save(p, v1).unwrap() {
            SaveOutcome::Stale { stored, stored_rev } => {
                assert_eq!(stored, v2);
                assert_eq!(stored_rev, StoreRev(2));
            }
            SaveOutcome::Saved(_) => panic!("stale rev must not clobber"),
        }
    }

    #[test]
    fn rejects_bad_profiles_and_bodies() {
        let s = store("rejects");
        // Path-traversal-shaped and short ids are refused before touching disk.
        assert!(s.load("../../etc/passwd").is_err());
        assert!(s.load("short").is_err());
        assert!(s.save("has space in it", r#"{"rev":1}"#).is_err());
        // Non-JSON and rev-less bodies are refused.
        let p = "aaaaaaaaaaaaaaaa";
        assert!(s.save(p, "not json").is_err());
        assert!(s.save(p, r#"{"norev":true}"#).is_err());
        // Oversized bodies are refused.
        let huge = format!(r#"{{"rev":1,"pad":"{}"}}"#, "x".repeat(MAX_STORE_BYTES));
        assert!(s.save(p, &huge).is_err());
    }

    #[test]
    fn disabled_store_reports_cleanly() {
        let s = DeckStore::new(None);
        assert!(!s.enabled());
        assert!(s.load("aaaaaaaaaaaaaaaa").is_err());
    }
}
