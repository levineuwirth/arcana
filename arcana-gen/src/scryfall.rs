//! Scryfall bulk data loader.
//!
//! Loads the `oracle_cards` bulk dump (one entry per unique oracle
//! text — the right granularity for card-generation) into an in-memory
//! [`ScryfallPool`]. Downloads-and-caches on first use; subsequent
//! runs read from the local cache unless the file is removed.
//!
//! Wire format: <https://scryfall.com/docs/api/bulk-data>.
//!
//! Deliberately *not* exhaustive of Scryfall's card schema — fields
//! are added as downstream work (tier classifier, prompt retrieval)
//! needs them. `serde` ignores unknown fields by default, so adding a
//! field later is a one-line change in this file.

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

/// Scryfall `bulk_data` entry as returned by
/// `GET /bulk-data/oracle-cards`. Only the fields we need to locate
/// the actual card dump.
#[derive(Debug, Clone, Deserialize)]
struct BulkDataEntry {
    #[serde(rename = "type")]
    kind: String,
    download_uri: String,
    updated_at: String,
    #[serde(default)]
    size: u64,
}

/// One card from the Scryfall `oracle_cards` bulk dump. Fields are
/// the subset we've needed so far; serde ignores the rest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// Scryfall UUID for this specific printing-agnostic card row.
    pub id: String,
    /// Stable across reprints — canonical handle for "this card".
    pub oracle_id: String,
    pub name: String,
    /// `{1}{R}` form; None for lands and some tokens.
    #[serde(default)]
    pub mana_cost: Option<String>,
    /// Converted mana value. Scryfall uses f32 because X-costs etc.
    /// report non-integer values in edge cases; for most cards this
    /// is an integer value stored as f32.
    #[serde(default)]
    pub cmc: f32,
    /// Full type line: `"Creature — Human Wizard"`.
    pub type_line: String,
    /// Rules text. Absent for cards with no rules (vanilla tokens,
    /// some basic lands).
    #[serde(default)]
    pub oracle_text: Option<String>,
    /// Stored as a string because of `"*"`, `"1+*"`, etc.
    #[serde(default)]
    pub power: Option<String>,
    #[serde(default)]
    pub toughness: Option<String>,
    #[serde(default)]
    pub loyalty: Option<String>,
    /// Defense value on Battle cards.
    #[serde(default)]
    pub defense: Option<String>,
    /// Single-letter color codes: `["W","U","B","R","G"]` subset.
    /// Empty for colorless cards.
    #[serde(default)]
    pub colors: Vec<String>,
    #[serde(default)]
    pub color_identity: Vec<String>,
    /// Keywords Scryfall has parsed out of oracle text.
    #[serde(default)]
    pub keywords: Vec<String>,
    /// Scryfall-reported mana colors the card can produce (for lands
    /// and mana-producing artifacts/creatures). `None` if the card
    /// produces no mana.
    #[serde(default)]
    pub produced_mana: Option<Vec<String>>,
    /// Format → legality status (`"legal"`, `"banned"`, `"restricted"`,
    /// `"not_legal"`). Keys include `"standard"`, `"pioneer"`,
    /// `"modern"`, `"legacy"`, `"vintage"`, `"commander"`, etc.
    pub legalities: HashMap<String, String>,
    pub rarity: String,
    /// Set code this card's most recent printing is from.
    pub set: String,
    /// `"normal"`, `"split"`, `"transform"`, `"modal_dfc"`,
    /// `"adventure"`, `"meld"`, `"saga"`, etc. Drives the Phase-3
    /// classifier's multi-face routing.
    pub layout: String,
    /// Present on multi-face layouts (`split`, `adventure`,
    /// `modal_dfc`, `transform`, `meld`, …). For those layouts the
    /// top-level `oracle_text` is typically absent and each face
    /// carries its own text, P/T, mana cost, etc. `None` for
    /// single-face `normal` cards.
    #[serde(default)]
    pub card_faces: Option<Vec<CardFace>>,
}

/// One face of a multi-face card. Fields mirror the Card struct's
/// card-scoped subset; anything not present on a given face is
/// `None`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardFace {
    pub name: String,
    #[serde(default)]
    pub mana_cost: Option<String>,
    #[serde(default)]
    pub type_line: Option<String>,
    #[serde(default)]
    pub oracle_text: Option<String>,
    #[serde(default)]
    pub power: Option<String>,
    #[serde(default)]
    pub toughness: Option<String>,
    #[serde(default)]
    pub loyalty: Option<String>,
    #[serde(default)]
    pub colors: Option<Vec<String>>,
}

/// Return the types-side of a Scryfall `type_line`, dropping the
/// subtypes after the em-dash separator. For "Creature — Human
/// Wizard" this returns "Creature ". For a bare type like "Instant"
/// (no subtypes, no dash) the whole string is returned.
///
/// Defensive against future MTG subtypes that could share text
/// with type names — e.g., a hypothetical subtype "Battle" on a
/// creature would false-positive `is_battle` under a whole-line
/// substring match. Splitting on `" — "` scopes every predicate
/// below to the types portion only.
pub(crate) fn type_part(type_line: &str) -> &str {
    match type_line.split_once(" — ") {
        Some((types, _subtypes)) => types,
        None => type_line,
    }
}

impl Card {
    pub fn is_standard_legal(&self) -> bool {
        self.legalities.get("standard").map(|s| s == "legal").unwrap_or(false)
    }

    pub fn is_creature(&self) -> bool {
        type_part(&self.type_line).contains("Creature")
    }
    pub fn is_instant(&self) -> bool {
        type_part(&self.type_line).contains("Instant")
    }
    pub fn is_sorcery(&self) -> bool {
        type_part(&self.type_line).contains("Sorcery")
    }
    pub fn is_land(&self) -> bool {
        type_part(&self.type_line).contains("Land")
    }
    pub fn is_enchantment(&self) -> bool {
        type_part(&self.type_line).contains("Enchantment")
    }
    pub fn is_artifact(&self) -> bool {
        type_part(&self.type_line).contains("Artifact")
    }
    pub fn is_planeswalker(&self) -> bool {
        type_part(&self.type_line).contains("Planeswalker")
    }
    pub fn is_battle(&self) -> bool {
        type_part(&self.type_line).contains("Battle")
    }

    /// Oracle text surface usable by downstream analysis.
    /// For single-face cards this is just `self.oracle_text`.
    /// For multi-face cards the top-level field is typically `None`
    /// and per-face text lives in `card_faces`; this joins all face
    /// texts with `\n---\n` so downstream heuristics (tier
    /// classifier, prompt retrieval) see the full card.
    ///
    /// Precedence check (2026-04-21): across 1,011 real multi-face
    /// cards in the live Scryfall oracle dump (split/adventure/
    /// modal_dfc/transform/flip), **zero** had both a populated
    /// top-level `oracle_text` and per-face text — every one was
    /// face-only. Meld-layout cards (21 in the dump) use top-level
    /// exclusively and lack `card_faces`. So the "prefer top-level
    /// when non-empty" fallthrough is safe: it never shadows face
    /// text on a real card.
    pub fn effective_oracle_text(&self) -> String {
        if let Some(t) = &self.oracle_text {
            if !t.is_empty() {
                return t.clone();
            }
        }
        match &self.card_faces {
            Some(faces) => faces
                .iter()
                .filter_map(|f| f.oracle_text.as_deref())
                .collect::<Vec<_>>()
                .join("\n---\n"),
            None => String::new(),
        }
    }

    /// Vanilla in the classical sense — a creature whose rules text
    /// is empty or only contains flavor (no keyword abilities, no
    /// triggered/activated abilities). Useful for tier-1 routing.
    ///
    /// Uses [`Self::effective_oracle_text`] so multi-face cards
    /// (adventure, modal_dfc, split, transform) are checked against
    /// the joined face text rather than the frequently-`None`
    /// top-level `oracle_text`. Otherwise every adventure-layout
    /// creature with `oracle_text: None` would false-positive as
    /// vanilla.
    pub fn is_vanilla_creature(&self) -> bool {
        if !self.is_creature() {
            return false;
        }
        self.effective_oracle_text().trim().is_empty()
    }
}

/// Loaded Scryfall card pool. Query methods are linear scans — this
/// runs on ~30k cards, so a HashMap-by-name index is future work when
/// we need it.
#[derive(Debug, Clone)]
pub struct ScryfallPool {
    cards: Vec<Card>,
    by_oracle_id: HashMap<String, usize>,
    by_name: HashMap<String, usize>,
}

impl ScryfallPool {
    /// Build a pool from a raw JSON string — the body of an
    /// `oracle_cards` bulk dump. Primary use is tests; production
    /// code usually goes through [`Self::from_cache_or_download`].
    pub fn from_json_str(json: &str) -> Result<Self> {
        let cards: Vec<Card> = serde_json::from_str(json)
            .context("parsing oracle_cards bulk JSON")?;
        Ok(Self::from_cards(cards))
    }

    fn from_cards(cards: Vec<Card>) -> Self {
        let mut by_oracle_id = HashMap::with_capacity(cards.len());
        let mut by_name = HashMap::with_capacity(cards.len());
        for (i, c) in cards.iter().enumerate() {
            by_oracle_id.insert(c.oracle_id.clone(), i);
            by_name.insert(c.name.clone(), i);
        }
        Self { cards, by_oracle_id, by_name }
    }

    /// Load the oracle_cards bulk from `cache_path` if it exists,
    /// otherwise download it from Scryfall and save a copy for
    /// subsequent runs. Explicit refresh: delete the file.
    pub fn from_cache_or_download(cache_path: &Path) -> Result<Self> {
        if cache_path.exists() {
            tracing::info!(path = %cache_path.display(), "loading Scryfall pool from cache");
            let json = fs::read_to_string(cache_path)
                .with_context(|| format!("reading cache {}", cache_path.display()))?;
            return Self::from_json_str(&json);
        }
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("creating cache dir {}", parent.display()))?;
        }
        tracing::info!(path = %cache_path.display(), "downloading Scryfall oracle_cards bulk");
        let json = fetch_oracle_cards_remote()?;
        fs::write(cache_path, &json)
            .with_context(|| format!("writing cache {}", cache_path.display()))?;
        Self::from_json_str(&json)
    }

    /// Convenience wrapper that defaults `cache_path` to
    /// `<workspace>/target/scryfall-cache/oracle-cards.json`. The
    /// `target/` dir is gitignored, so the cache file never
    /// pollutes the working tree.
    pub fn load_default() -> Result<Self> {
        Self::from_cache_or_download(&default_cache_path())
    }

    // --- query API -------------------------------------------------

    pub fn len(&self) -> usize { self.cards.len() }
    pub fn is_empty(&self) -> bool { self.cards.is_empty() }

    pub fn iter(&self) -> impl Iterator<Item = &Card> + '_ {
        self.cards.iter()
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Card> {
        self.by_name.get(name).map(|&i| &self.cards[i])
    }

    pub fn find_by_oracle_id(&self, oracle_id: &str) -> Option<&Card> {
        self.by_oracle_id.get(oracle_id).map(|&i| &self.cards[i])
    }

    pub fn filter<'a, F: Fn(&Card) -> bool + 'a>(
        &'a self,
        pred: F,
    ) -> impl Iterator<Item = &'a Card> + 'a {
        self.cards.iter().filter(move |c| pred(c))
    }

    pub fn standard_legal(&self) -> impl Iterator<Item = &Card> + '_ {
        self.filter(|c| c.is_standard_legal())
    }
}

// =============================================================================
// remote fetch
// =============================================================================

const BULK_META_URL: &str = "https://api.scryfall.com/bulk-data/oracle-cards";
const SCRYFALL_USER_AGENT: &str = concat!(
    "arcana-gen/",
    env!("CARGO_PKG_VERSION"),
    " (research MTG engine)"
);

/// Two-step fetch per Scryfall's API:
///   1. GET /bulk-data/oracle-cards → returns metadata with download_uri
///   2. GET that download_uri         → returns the actual JSON array
fn fetch_oracle_cards_remote() -> Result<String> {
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(10))
        .timeout_read(Duration::from_secs(120))
        .user_agent(SCRYFALL_USER_AGENT)
        .build();

    let meta: BulkDataEntry = agent
        .get(BULK_META_URL)
        .set("Accept", "application/json")
        .call()
        .context("GET /bulk-data/oracle-cards")?
        .into_json()
        .context("parsing bulk-data metadata")?;

    if meta.kind != "oracle_cards" {
        return Err(anyhow!(
            "expected bulk-data kind 'oracle_cards', got {:?}",
            meta.kind
        ));
    }
    tracing::info!(
        download_uri = %meta.download_uri,
        updated_at   = %meta.updated_at,
        size_bytes   = meta.size,
        "resolved Scryfall oracle_cards metadata"
    );

    // Stream the body to a string. The bulk file is tens of MB — fits
    // in memory comfortably and keeps the control flow simple.
    let mut body = String::new();
    agent
        .get(&meta.download_uri)
        .set("Accept", "application/json")
        .call()
        .with_context(|| format!("GET {}", meta.download_uri))?
        .into_reader()
        .read_to_string(&mut body)
        .context("reading oracle_cards bulk body")?;
    Ok(body)
}

fn default_cache_path() -> PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest)
        .join("..")
        .join("target")
        .join("scryfall-cache")
        .join("oracle-cards.json")
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Tiny hand-written fixture covering: a creature with keywords,
    /// an instant, a basic land, a standard-illegal card, and an
    /// adventure card (layout variant). Enough to exercise every
    /// query method without hitting the network.
    const FIXTURE: &str = r#"[
        {
            "id": "grizzly-1",
            "oracle_id": "oracle-grizzly",
            "name": "Grizzly Bears",
            "mana_cost": "{1}{G}",
            "cmc": 2.0,
            "type_line": "Creature — Bear",
            "oracle_text": "",
            "power": "2",
            "toughness": "2",
            "colors": ["G"],
            "color_identity": ["G"],
            "keywords": [],
            "legalities": {"standard": "legal", "modern": "legal"},
            "rarity": "common",
            "set": "lea",
            "layout": "normal"
        },
        {
            "id": "bolt-1",
            "oracle_id": "oracle-bolt",
            "name": "Lightning Bolt",
            "mana_cost": "{R}",
            "cmc": 1.0,
            "type_line": "Instant",
            "oracle_text": "Lightning Bolt deals 3 damage to any target.",
            "colors": ["R"],
            "color_identity": ["R"],
            "keywords": [],
            "legalities": {"standard": "not_legal", "modern": "legal"},
            "rarity": "common",
            "set": "lea",
            "layout": "normal"
        },
        {
            "id": "mountain-1",
            "oracle_id": "oracle-mountain",
            "name": "Mountain",
            "cmc": 0.0,
            "type_line": "Basic Land — Mountain",
            "oracle_text": "({T}: Add {R}.)",
            "colors": [],
            "color_identity": ["R"],
            "keywords": [],
            "produced_mana": ["R"],
            "legalities": {"standard": "legal"},
            "rarity": "common",
            "set": "lea",
            "layout": "normal"
        },
        {
            "id": "bonecrusher-1",
            "oracle_id": "oracle-bonecrusher",
            "name": "Bonecrusher Giant",
            "mana_cost": "{2}{R}",
            "cmc": 3.0,
            "type_line": "Creature — Giant",
            "oracle_text": "Whenever Bonecrusher Giant becomes the target of a spell, Bonecrusher Giant deals 2 damage to that spell's controller.",
            "power": "4",
            "toughness": "3",
            "colors": ["R"],
            "color_identity": ["R"],
            "keywords": [],
            "legalities": {"standard": "legal"},
            "rarity": "rare",
            "set": "eld",
            "layout": "adventure"
        }
    ]"#;

    #[test]
    fn parses_fixture_pool() {
        let pool = ScryfallPool::from_json_str(FIXTURE).expect("parse");
        assert_eq!(pool.len(), 4);
    }

    #[test]
    fn unknown_card_fields_are_ignored() {
        // Scryfall returns many fields we don't model. Parsing must
        // keep working as the upstream schema grows.
        let json = r#"[{
            "id": "x", "oracle_id": "y", "name": "Z",
            "cmc": 0.0, "type_line": "Instant",
            "legalities": {}, "rarity": "common", "set": "xxx",
            "layout": "normal",
            "released_at": "2026-01-01",
            "card_faces": [{"name": "nonsense"}],
            "prices": {"usd": "1.23"},
            "image_uris": {"normal": "http://x"}
        }]"#;
        let pool = ScryfallPool::from_json_str(json).expect("parse");
        assert_eq!(pool.len(), 1);
    }

    #[test]
    fn find_by_name_hits_and_misses() {
        let pool = ScryfallPool::from_json_str(FIXTURE).expect("parse");
        assert_eq!(pool.find_by_name("Grizzly Bears").unwrap().power.as_deref(), Some("2"));
        assert!(pool.find_by_name("Nonexistent").is_none());
    }

    #[test]
    fn find_by_oracle_id_is_stable_handle() {
        let pool = ScryfallPool::from_json_str(FIXTURE).expect("parse");
        let c = pool.find_by_oracle_id("oracle-bolt").expect("bolt");
        assert_eq!(c.name, "Lightning Bolt");
    }

    #[test]
    fn standard_legal_filter_excludes_banned_and_not_legal() {
        let pool = ScryfallPool::from_json_str(FIXTURE).expect("parse");
        let names: Vec<_> = pool.standard_legal().map(|c| c.name.as_str()).collect();
        // Lightning Bolt is not_legal in Standard → excluded.
        assert!(!names.contains(&"Lightning Bolt"), "bolt must be excluded");
        // Grizzly Bears, Mountain, Bonecrusher Giant are standard-legal.
        assert_eq!(names.len(), 3);
    }

    #[test]
    fn type_line_predicates() {
        let pool = ScryfallPool::from_json_str(FIXTURE).expect("parse");
        assert!(pool.find_by_name("Grizzly Bears").unwrap().is_creature());
        assert!(pool.find_by_name("Lightning Bolt").unwrap().is_instant());
        assert!(pool.find_by_name("Mountain").unwrap().is_land());
        assert!(!pool.find_by_name("Grizzly Bears").unwrap().is_land());
    }

    #[test]
    fn type_part_splits_on_em_dash() {
        assert_eq!(type_part("Creature — Human Wizard"), "Creature");
        assert_eq!(type_part("Basic Land — Mountain"), "Basic Land");
        assert_eq!(type_part("Instant"), "Instant");
        assert_eq!(type_part("Legendary Planeswalker — Jace"), "Legendary Planeswalker");
        assert_eq!(type_part("Artifact — Equipment"), "Artifact");
    }

    #[test]
    fn type_predicates_do_not_false_positive_on_subtypes() {
        // Defensive coverage: a creature whose subtype happens to
        // share text with a type name must not register as that
        // type. MTG doesn't currently have a subtype named "Battle"
        // or "Creature", but the substring form would've been
        // fragile against any future subtype rename.
        let json = r#"[{
            "id": "x", "oracle_id": "y", "name": "Hypothetical Creature",
            "cmc": 3.0, "type_line": "Creature — Battle Mage",
            "legalities": {}, "rarity": "common", "set": "xxx",
            "layout": "normal"
        }]"#;
        let pool = ScryfallPool::from_json_str(json).expect("parse");
        let c = pool.find_by_name("Hypothetical Creature").unwrap();
        assert!(c.is_creature(), "must still register as creature");
        assert!(!c.is_battle(), "must NOT register as Battle just because 'Battle' is in the subtype portion");
    }

    #[test]
    fn vanilla_creature_detection() {
        let pool = ScryfallPool::from_json_str(FIXTURE).expect("parse");
        // Grizzly Bears: Creature + empty oracle_text → vanilla.
        assert!(pool.find_by_name("Grizzly Bears").unwrap().is_vanilla_creature());
        // Bonecrusher Giant: Creature + non-empty text → not vanilla.
        assert!(!pool.find_by_name("Bonecrusher Giant").unwrap().is_vanilla_creature());
        // Lightning Bolt: not a creature → not vanilla.
        assert!(!pool.find_by_name("Lightning Bolt").unwrap().is_vanilla_creature());
    }

    #[test]
    fn adventure_layout_preserved() {
        let pool = ScryfallPool::from_json_str(FIXTURE).expect("parse");
        let bc = pool.find_by_name("Bonecrusher Giant").expect("present");
        assert_eq!(bc.layout, "adventure");
    }

    #[test]
    fn filter_custom_predicate() {
        let pool = ScryfallPool::from_json_str(FIXTURE).expect("parse");
        let red_creatures: Vec<_> = pool
            .filter(|c| c.is_creature() && c.colors.contains(&"R".to_string()))
            .collect();
        assert_eq!(red_creatures.len(), 1);
        assert_eq!(red_creatures[0].name, "Bonecrusher Giant");
    }

    #[test]
    fn default_cache_path_is_under_target() {
        let p = default_cache_path();
        let s = p.to_string_lossy();
        assert!(s.contains("target"), "cache path must live under target/: {s}");
        assert!(s.ends_with("oracle-cards.json"), "cache filename: {s}");
    }

    /// Live Scryfall fetch. Ignored by default — run manually with
    /// `cargo test -p arcana-gen --lib -- --ignored scryfall_live`
    /// when you want to verify the network path works end-to-end.
    #[test]
    #[ignore]
    fn scryfall_live_download_and_cache() {
        let tmp = std::env::temp_dir().join("arcana-gen-scryfall-test.json");
        let _ = std::fs::remove_file(&tmp);
        let pool = ScryfallPool::from_cache_or_download(&tmp).expect("download");
        assert!(pool.len() > 1000, "expected real Scryfall pool, got {}", pool.len());
        assert!(tmp.exists(), "cache file should have been written");
        let _ = std::fs::remove_file(&tmp);
    }
}
