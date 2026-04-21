//! Card complexity tier classifier.
//!
//! Routes a [`Card`] into one of five tiers that drive the Phase-3
//! prompt selection. Heuristic-based: fast, deterministic, explainable.
//!
//! Design principle: the classifier is a *prompt router*, not an
//! oracle. A card misclassified as T2 that actually needs T3
//! complexity will fail its smoke test under the T2 prompt and get
//! retried at T3. We don't try to perfectly predict generation
//! success — we try to route to a reasonable starting prompt.
//!
//! Gate order (most-restrictive first):
//!   1. T5 if the engine doesn't support the layout (meld, saga,
//!      battle, flip, …).
//!   2. T1 if basic land or truly vanilla creature.
//!   3. T4 for structural complexity signals (planeswalker, X cost,
//!      modal, multiple ability lines).
//!   4. T3 if oracle text contains a triggered or activated ability.
//!   5. T2 if french-vanilla creature or recognizable single-effect
//!      instant/sorcery.
//!   6. T5 fallback when no heuristic matched.

use crate::scryfall::Card;

/// Complexity tier for prompt routing. Higher tier = more complex
/// expected prompt + retry budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tier {
    /// Pure vanilla: basic lands, creatures with no rules text.
    One,
    /// Single effect: Lightning Bolt, Grizzly Bears, french-vanilla
    /// creatures (keywords only, no other text).
    Two,
    /// Composed: one triggered or activated ability. Elvish
    /// Visionary (ETB draw), Snapcaster Mage (ETB target),
    /// Bonesplitter equip.
    Three,
    /// Complex: multiple abilities, planeswalkers, modal spells,
    /// X costs.
    Four,
    /// Triage — unsupported layout or unrecognized structure.
    /// Human review + manual prompt engineering before retry.
    Five,
}

impl Tier {
    pub fn as_number(self) -> u8 {
        match self {
            Tier::One => 1,
            Tier::Two => 2,
            Tier::Three => 3,
            Tier::Four => 4,
            Tier::Five => 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Classification {
    pub tier: Tier,
    /// Human-readable reason for the assignment. Used for debugging
    /// the classifier on surprising cards and for logging tier
    /// distributions over the full pool.
    pub rationale: String,
}

impl Classification {
    fn new(tier: Tier, rationale: impl Into<String>) -> Self {
        Self { tier, rationale: rationale.into() }
    }
}

/// Top-level classifier.
pub fn classify(card: &Card) -> Classification {
    if let Some(layout) = unsupported_layout(card) {
        return Classification::new(
            Tier::Five,
            format!("unsupported layout: {layout}"),
        );
    }

    if is_basic_land(card) {
        return Classification::new(Tier::One, "basic land");
    }
    if card.is_vanilla_creature() {
        return Classification::new(Tier::One, "vanilla creature (no rules text)");
    }

    if card.is_planeswalker() {
        return Classification::new(Tier::Four, "planeswalker (multiple loyalty abilities)");
    }
    if has_x_cost(card) {
        return Classification::new(Tier::Four, "X mana cost");
    }
    let text = card.effective_oracle_text();
    if is_modal(&text) {
        return Classification::new(Tier::Four, "modal spell (choose one / choose two)");
    }
    if has_multiple_ability_lines(&text) {
        return Classification::new(Tier::Four, "multiple ability lines");
    }

    if has_triggered_ability(&text) {
        return Classification::new(Tier::Three, "triggered ability");
    }
    if has_activated_ability(&text) {
        return Classification::new(Tier::Three, "activated ability");
    }

    if is_french_vanilla_creature(card, &text) {
        return Classification::new(Tier::Two, "french-vanilla creature (keywords only)");
    }
    if matches_single_effect_spell(card, &text) {
        return Classification::new(Tier::Two, "single-effect instant/sorcery");
    }

    Classification::new(
        Tier::Five,
        "no matching heuristic — manual review needed",
    )
}

// =============================================================================
// heuristics
// =============================================================================

/// Layouts the engine doesn't yet have machinery for. Returns the
/// offending layout string if the card uses one. The list mirrors
/// arcana-core's Phase-1 status — keep in sync as engine features
/// land.
fn unsupported_layout(c: &Card) -> Option<&str> {
    match c.layout.as_str() {
        // Supported today: normal, split, adventure, modal_dfc.
        "normal" | "split" | "adventure" | "modal_dfc" => None,

        // Deferred per SBA 704.5s–u or never-implemented.
        "meld" | "leveler" | "class" | "saga" | "battle" | "flip" | "planar"
        | "scheme" | "vanguard" | "augment" | "host" | "transform" | "case"
        // Non-card placeholders (tokens on the card-face list, emblem rows,
        // art-only entries). We shouldn't classify these at all — they
        // shouldn't be in a card-gen corpus.
        | "token" | "emblem" | "double_faced_token" | "art_series"
        | "reversible_card" => Some(&c.layout),

        // Anything we haven't seen yet: route to triage rather than
        // silently misroute. Cheap to extend this match when a new
        // layout shows up.
        _ => Some(&c.layout),
    }
}

fn is_basic_land(c: &Card) -> bool {
    c.type_line.contains("Basic Land")
}

fn has_x_cost(c: &Card) -> bool {
    c.mana_cost
        .as_deref()
        .map(|s| s.contains("{X}"))
        .unwrap_or(false)
}

fn is_modal(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("choose one")
        || lower.contains("choose two")
        || lower.contains("choose up to")
}

/// True when the card has more than one ability line. Scryfall
/// separates ability paragraphs with a single `\n`. Reminder text
/// (inside parens) still sits on the same line as its keyword, so
/// it doesn't inflate this count.
fn has_multiple_ability_lines(text: &str) -> bool {
    text.lines().filter(|l| !l.trim().is_empty()).count() >= 2
}

fn has_triggered_ability(text: &str) -> bool {
    let stripped = strip_reminder_text(text).to_lowercase();
    // "When …", "Whenever …", "At the beginning of …",
    // "At the end of …". Anchored with a leading space (or start of
    // text) so we don't match "even when" mid-sentence.
    for trigger in ["when ", "whenever ", "at the beginning", "at the end"] {
        if stripped.starts_with(trigger) || stripped.contains(&format!(" {trigger}")) {
            return true;
        }
    }
    false
}

/// Activated abilities on printed cards almost always have the
/// form `<cost>: <effect>`. Heuristic: scan each line for a `": "`
/// whose prefix looks cost-like — contains a mana symbol (`{…}`)
/// or is a short sacrifice / discard / pay-life clause. Reminder
/// text in parens is stripped first so "Equip {1} (…)" doesn't
/// look like two abilities.
fn has_activated_ability(text: &str) -> bool {
    let stripped = strip_reminder_text(text);
    for line in stripped.lines() {
        let Some(colon) = line.find(": ") else { continue };
        let prefix = line[..colon].trim();
        if prefix.is_empty() {
            continue;
        }
        let plower = prefix.to_lowercase();
        let looks_like_cost = prefix.contains('{')
            || plower.starts_with("sacrifice ")
            || plower.starts_with("discard ")
            || plower.starts_with("pay ")
            || plower.starts_with("exile ")
            // Short costless prefix — catches "Equip {1}:" patterns
            // after the mana-symbol branch, and a few keyword-ability
            // costs whose written form starts with the keyword name.
            || (prefix.len() <= 20 && !plower.contains("when"));
        if looks_like_cost {
            return true;
        }
    }
    false
}

/// French-vanilla: creature whose only rules text is a list of
/// keywords (`Flying`, `First strike`, …) plus their reminder text.
/// Distinct from true vanilla (empty oracle text → T1).
///
/// Heuristic: strip reminder text, then verify that every
/// alphabetic word remaining is either a word in one of the
/// Scryfall-parsed keyword names, or a connective (`and`, `or`).
/// Non-creatures are excluded up front. Creatures with no
/// Scryfall-detected keywords are excluded (can't be french vanilla
/// if keywords is empty).
fn is_french_vanilla_creature(c: &Card, text: &str) -> bool {
    if !c.is_creature() {
        return false;
    }
    if text.trim().is_empty() {
        return false;
    }
    if c.keywords.is_empty() {
        return false;
    }

    let stripped = strip_reminder_text(text).to_lowercase();

    let text_words: Vec<&str> = stripped
        .split(|ch: char| !ch.is_alphabetic())
        .filter(|s| !s.is_empty())
        .collect();

    // Flatten keyword list into individual words — "first strike",
    // "double strike", etc. all contribute their component words.
    let kw_words: std::collections::HashSet<String> = c
        .keywords
        .iter()
        .flat_map(|k| {
            k.to_lowercase()
                .split_whitespace()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
        })
        .collect();

    const CONNECTIVES: &[&str] = &["and", "or"];

    text_words
        .iter()
        .all(|w| kw_words.contains(*w) || CONNECTIVES.contains(w))
}

/// Recognize common single-effect instant/sorcery shapes. By the
/// time we reach this check, we've already filtered T3 (triggered /
/// activated) and T4 (modal / X / multi-line). So this is a coarse
/// "is it a straightforward spell" predicate.
fn matches_single_effect_spell(c: &Card, text: &str) -> bool {
    if !(c.is_instant() || c.is_sorcery()) {
        return false;
    }
    if text.trim().is_empty() {
        return false;
    }
    let lower = text.to_lowercase();
    const T2_VERBS: &[&str] = &[
        "deals ",
        "destroy ",
        "counter ",
        "exile ",
        "draw ",
        "discard",
        "return ",
        "gets +",
        "gets -",
        "create ",
        "add {",
        "tap target",
        "untap target",
        "search your library",
    ];
    T2_VERBS.iter().any(|v| lower.contains(v))
}

fn strip_reminder_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut depth: u32 = 0;
    for ch in s.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(ch),
            _ => {}
        }
    }
    out
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Build a test Card with sensible defaults and the given
    /// overrides applied via a closure. Saves us writing 15-field
    /// struct literals in every test.
    fn mk_card(configure: impl FnOnce(&mut Card)) -> Card {
        let mut c = Card {
            id: "test-id".into(),
            oracle_id: "test-oracle".into(),
            name: "Test Card".into(),
            mana_cost: Some("{1}".into()),
            cmc: 1.0,
            type_line: "Instant".into(),
            oracle_text: Some(String::new()),
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
            colors: vec![],
            color_identity: vec![],
            keywords: vec![],
            produced_mana: None,
            legalities: {
                let mut m = HashMap::new();
                m.insert("standard".into(), "legal".into());
                m
            },
            rarity: "common".into(),
            set: "tst".into(),
            layout: "normal".into(),
            card_faces: None,
        };
        configure(&mut c);
        c
    }

    // --- T1 --------------------------------------------------------

    #[test]
    fn t1_basic_land() {
        let c = mk_card(|c| {
            c.name = "Mountain".into();
            c.type_line = "Basic Land — Mountain".into();
            c.oracle_text = Some("({T}: Add {R}.)".into());
            c.mana_cost = None;
            c.cmc = 0.0;
        });
        assert_eq!(classify(&c).tier, Tier::One);
    }

    #[test]
    fn t1_vanilla_creature() {
        let c = mk_card(|c| {
            c.name = "Grizzly Bears".into();
            c.type_line = "Creature — Bear".into();
            c.oracle_text = Some(String::new());
            c.power = Some("2".into());
            c.toughness = Some("2".into());
        });
        assert_eq!(classify(&c).tier, Tier::One);
    }

    // --- T2 --------------------------------------------------------

    #[test]
    fn t2_simple_damage_instant() {
        let c = mk_card(|c| {
            c.name = "Lightning Bolt".into();
            c.type_line = "Instant".into();
            c.oracle_text = Some("Lightning Bolt deals 3 damage to any target.".into());
        });
        assert_eq!(classify(&c).tier, Tier::Two);
    }

    #[test]
    fn t2_simple_destroy_sorcery() {
        let c = mk_card(|c| {
            c.name = "Murder".into();
            c.type_line = "Instant".into();
            c.oracle_text = Some("Destroy target creature.".into());
        });
        assert_eq!(classify(&c).tier, Tier::Two);
    }

    #[test]
    fn t2_french_vanilla_creature_single_keyword() {
        let c = mk_card(|c| {
            c.name = "Wind Drake".into();
            c.type_line = "Creature — Drake".into();
            c.oracle_text = Some("Flying".into());
            c.keywords = vec!["Flying".into()];
            c.power = Some("2".into());
            c.toughness = Some("2".into());
        });
        let cls = classify(&c);
        assert_eq!(cls.tier, Tier::Two, "rationale={}", cls.rationale);
    }

    #[test]
    fn t2_french_vanilla_creature_multi_keyword() {
        let c = mk_card(|c| {
            c.name = "Serra Angel".into();
            c.type_line = "Creature — Angel".into();
            c.oracle_text = Some("Flying, vigilance".into());
            c.keywords = vec!["Flying".into(), "Vigilance".into()];
            c.power = Some("4".into());
            c.toughness = Some("4".into());
        });
        let cls = classify(&c);
        assert_eq!(cls.tier, Tier::Two, "rationale={}", cls.rationale);
    }

    #[test]
    fn t2_french_vanilla_with_reminder_text() {
        // The reminder-text stripper must let keyword-only text pass
        // as french vanilla, even when Scryfall includes the italic
        // reminder block.
        let c = mk_card(|c| {
            c.name = "Test Flier".into();
            c.type_line = "Creature — Bird".into();
            c.oracle_text = Some(
                "Flying (This creature can't be blocked except by creatures with flying or reach.)"
                    .into(),
            );
            c.keywords = vec!["Flying".into()];
            c.power = Some("1".into());
            c.toughness = Some("1".into());
        });
        let cls = classify(&c);
        assert_eq!(cls.tier, Tier::Two, "rationale={}", cls.rationale);
    }

    // --- T3 --------------------------------------------------------

    #[test]
    fn t3_etb_trigger() {
        let c = mk_card(|c| {
            c.name = "Elvish Visionary".into();
            c.type_line = "Creature — Elf Shaman".into();
            c.oracle_text = Some("When Elvish Visionary enters, draw a card.".into());
            c.power = Some("1".into());
            c.toughness = Some("1".into());
        });
        let cls = classify(&c);
        assert_eq!(cls.tier, Tier::Three, "rationale={}", cls.rationale);
    }

    #[test]
    fn t3_activated_ability() {
        let c = mk_card(|c| {
            c.name = "Icy Manipulator".into();
            c.type_line = "Artifact".into();
            c.oracle_text = Some("{1}, {T}: Tap target permanent.".into());
        });
        let cls = classify(&c);
        assert_eq!(cls.tier, Tier::Three, "rationale={}", cls.rationale);
    }

    #[test]
    fn t3_whenever_trigger() {
        let c = mk_card(|c| {
            c.name = "Young Pyromancer".into();
            c.type_line = "Creature — Human Shaman".into();
            c.oracle_text = Some(
                "Whenever you cast an instant or sorcery spell, create a 1/1 red Elemental creature token."
                    .into(),
            );
            c.power = Some("2".into());
            c.toughness = Some("1".into());
        });
        let cls = classify(&c);
        assert_eq!(cls.tier, Tier::Three, "rationale={}", cls.rationale);
    }

    // --- T4 --------------------------------------------------------

    #[test]
    fn t4_planeswalker() {
        let c = mk_card(|c| {
            c.name = "Chandra".into();
            c.type_line = "Legendary Planeswalker — Chandra".into();
            c.oracle_text = Some("+1: stuff.\n-3: more stuff.".into());
            c.loyalty = Some("4".into());
        });
        assert_eq!(classify(&c).tier, Tier::Four);
    }

    #[test]
    fn t4_x_cost() {
        let c = mk_card(|c| {
            c.name = "Fireball".into();
            c.mana_cost = Some("{X}{R}".into());
            c.type_line = "Sorcery".into();
            c.oracle_text = Some("Fireball deals X damage to any target.".into());
        });
        assert_eq!(classify(&c).tier, Tier::Four);
    }

    #[test]
    fn t4_modal_spell() {
        let c = mk_card(|c| {
            c.name = "Abrade".into();
            c.type_line = "Instant".into();
            c.oracle_text = Some(
                "Choose one —\n• Abrade deals 3 damage to target creature.\n• Destroy target artifact."
                    .into(),
            );
        });
        let cls = classify(&c);
        assert_eq!(cls.tier, Tier::Four, "rationale={}", cls.rationale);
    }

    #[test]
    fn t4_multiple_ability_lines() {
        let c = mk_card(|c| {
            c.name = "Bonesplitter".into();
            c.type_line = "Artifact — Equipment".into();
            c.oracle_text = Some("Equipped creature gets +3/+0.\nEquip {1}".into());
            c.keywords = vec!["Equip".into()];
        });
        assert_eq!(classify(&c).tier, Tier::Four);
    }

    // --- T5 --------------------------------------------------------

    #[test]
    fn t5_unsupported_layout_meld() {
        let c = mk_card(|c| {
            c.name = "Brisela".into();
            c.layout = "meld".into();
            c.type_line = "Legendary Creature — Angel".into();
            c.oracle_text = Some("Flying, vigilance, lifelink.".into());
        });
        let cls = classify(&c);
        assert_eq!(cls.tier, Tier::Five);
        assert!(cls.rationale.contains("meld"));
    }

    #[test]
    fn t5_unsupported_layout_saga() {
        let c = mk_card(|c| {
            c.layout = "saga".into();
        });
        assert_eq!(classify(&c).tier, Tier::Five);
    }

    #[test]
    fn t5_unsupported_layout_battle() {
        let c = mk_card(|c| {
            c.layout = "battle".into();
        });
        assert_eq!(classify(&c).tier, Tier::Five);
    }

    #[test]
    fn t5_unrecognized_fallback() {
        // Creature with non-keyword rules text that isn't a trigger
        // or activation: something like a static ability we haven't
        // heuristically caught yet.
        let c = mk_card(|c| {
            c.type_line = "Creature — Human".into();
            c.oracle_text = Some("Spells your opponents cast cost {1} more to cast.".into());
            c.power = Some("2".into());
            c.toughness = Some("2".into());
        });
        assert_eq!(classify(&c).tier, Tier::Five);
    }

    #[test]
    fn t5_unknown_layout_routes_to_triage() {
        let c = mk_card(|c| {
            c.layout = "some-new-layout-we-dont-recognize".into();
        });
        assert_eq!(classify(&c).tier, Tier::Five);
    }

    // --- multi-face ------------------------------------------------

    #[test]
    fn adventure_card_uses_effective_oracle_text() {
        use crate::scryfall::CardFace;
        // Bonecrusher Giant shape: the adventure face has oracle text
        // but the top-level oracle_text is usually absent. The
        // classifier should still see the text via card_faces.
        let c = mk_card(|c| {
            c.name = "Bonecrusher Giant // Stomp".into();
            c.layout = "adventure".into();
            c.type_line = "Creature — Giant".into();
            c.oracle_text = None;
            c.power = Some("4".into());
            c.toughness = Some("3".into());
            c.card_faces = Some(vec![
                CardFace {
                    name: "Bonecrusher Giant".into(),
                    mana_cost: Some("{2}{R}".into()),
                    type_line: Some("Creature — Giant".into()),
                    oracle_text: Some(
                        "Whenever Bonecrusher Giant becomes the target of a spell, Bonecrusher Giant deals 2 damage to that spell's controller."
                            .into(),
                    ),
                    power: Some("4".into()),
                    toughness: Some("3".into()),
                    loyalty: None,
                    colors: Some(vec!["R".into()]),
                },
                CardFace {
                    name: "Stomp".into(),
                    mana_cost: Some("{1}{R}".into()),
                    type_line: Some("Instant — Adventure".into()),
                    oracle_text: Some(
                        "Stomp deals 2 damage to any target.".into(),
                    ),
                    power: None,
                    toughness: None,
                    loyalty: None,
                    colors: Some(vec!["R".into()]),
                },
            ]);
        });
        // Adventure is a supported layout; the joined face text has
        // both a trigger AND a separate spell effect, joined with
        // `\n---\n` — so has_multiple_ability_lines fires first and
        // this routes to T4.
        let cls = classify(&c);
        assert_eq!(cls.tier, Tier::Four, "rationale={}", cls.rationale);
    }

    // --- tier ordering + utility -----------------------------------

    #[test]
    fn tier_is_orderable_by_complexity() {
        assert!(Tier::One < Tier::Two);
        assert!(Tier::Two < Tier::Three);
        assert!(Tier::Three < Tier::Four);
        assert!(Tier::Four < Tier::Five);
    }

    #[test]
    fn tier_number_matches_roman() {
        assert_eq!(Tier::One.as_number(), 1);
        assert_eq!(Tier::Five.as_number(), 5);
    }

    // --- stripper --------------------------------------------------

    #[test]
    fn strip_reminder_text_handles_parens() {
        assert_eq!(
            strip_reminder_text("Flying (This creature can't be blocked...)"),
            "Flying "
        );
        assert_eq!(strip_reminder_text("No parens here"), "No parens here");
        // Nested-paren edge case (rare in MTG but possible): the
        // depth counter handles it.
        assert_eq!(
            strip_reminder_text("Keyword (outer (nested) end)"),
            "Keyword "
        );
    }

    // --- live-pool tier distribution (ignored by default) ----------

    /// Runs the classifier against the live Scryfall oracle pool and
    /// prints a tier distribution to stderr. Ignored by default;
    /// useful sanity check when tuning heuristics. Run with:
    /// `cargo test -p arcana-gen --lib -- --ignored classifier_live`.
    #[test]
    #[ignore]
    fn classifier_live_tier_distribution() {
        use crate::scryfall::ScryfallPool;
        let tmp = std::env::temp_dir().join("arcana-gen-classify-test.json");
        let pool = ScryfallPool::from_cache_or_download(&tmp).expect("download");

        let mut counts = [0usize; 5];
        for card in pool.iter() {
            counts[classify(card).tier.as_number() as usize - 1] += 1;
        }
        let total: usize = counts.iter().sum();
        eprintln!("== tier distribution over {total} cards ==");
        for (i, n) in counts.iter().enumerate() {
            eprintln!(
                "  T{}: {:>6}  ({:.1}%)",
                i + 1,
                n,
                100.0 * *n as f64 / total as f64
            );
        }
        // Sanity only: every tier has at least one card in a real
        // Magic pool. If T1 is zero, basic-land detection broke.
        for (i, n) in counts.iter().enumerate() {
            assert!(*n > 0, "tier T{} is empty — heuristic regression?", i + 1);
        }
    }
}
