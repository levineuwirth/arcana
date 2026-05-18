//! Layer-2 structural verification: codegen.
//!
//! `verify::check` (layer 1) only proves a candidate *compiles*. A
//! file can compile and still be the wrong card — wrong colour,
//! wrong power, a keyword silently dropped. Layer 2 closes that gap
//! for the structurally-checkable subset (T1/T2): instantiate the
//! candidate's `register()`, pull the `CardDefinition` back out, and
//! diff its characteristics against the Scryfall row the prompt was
//! built from.
//!
//! # Why codegen instead of a checker binary
//!
//! The candidate lives in `arcana-cards`'s scratch module
//! (`generated::_scratch::candidate`). `arcana-cards` depends only
//! on `arcana-core` — it cannot call an `arcana-gen` helper. And a
//! standalone bin that statically referenced `candidate::register`
//! would fail to compile whenever the scratch holds the `_noop`
//! bootstrap stub (fresh clone, pre-verify), poisoning ordinary
//! `cargo build`. So layer 2 is a `#[cfg(test)]` module *appended
//! to the candidate source itself*, emitting self-contained
//! `arcana-core`-only assertions. `cargo check` (layer 1) skips
//! `#[cfg(test)]` entirely, so the two layers stay independent;
//! `cargo test` runs the appended `structural` test.
//!
//! # Scope / honesty
//!
//! This is deliberately a *fingerprint* diff, not a semantic proof.
//! It catches the failure modes that matter at T1/T2 — wrong
//! identity, colour, type, P/T, dropped evergreen keyword — and
//! stays silent on what it cannot check robustly (parametrised
//! keywords like `Ward(_)`, ability bodies, non-integer P/T). A
//! green layer 2 means "the card's bones match Scryfall", not "the
//! rules text is correctly implemented".

use crate::bakeoff::DumpRow;
use crate::scryfall::type_part;

/// The structural fingerprint a candidate must match. Built from a
/// [`DumpRow`]; rendered to Rust assertions by [`render_harness`].
#[derive(Debug, Clone)]
pub struct Expected {
    pub name: String,
    /// `Some(mv)` asserts `mana_cost` is present with that mana
    /// value; `None` asserts `mana_cost` is `None` (lands).
    pub mana_value: Option<u32>,
    /// W, U, B, R, G membership.
    pub colors: [bool; 5],
    pub is_creature: bool,
    pub is_instant: bool,
    pub is_sorcery: bool,
    pub is_artifact: bool,
    pub is_enchantment: bool,
    pub is_land: bool,
    pub is_planeswalker: bool,
    /// `Some(Some(n))` → assert `Fixed(n)`. `Some(None)` → assert
    /// the field is `None`. `None` → don't assert (non-integer
    /// P/T like `*`).
    pub power: Option<Option<i32>>,
    pub toughness: Option<Option<i32>>,
    /// Evergreen `KeywordAbility` unit-variant idents that must be
    /// present. Non-mappable Scryfall keywords are dropped here
    /// rather than risk a false negative — see module docs.
    pub keywords: Vec<&'static str>,
    /// Land-subtype names (`"Forest"`, …) the card must carry as
    /// `KeywordAbility::Landwalk(<interned subtype>)`. Asserted with a
    /// guarded match that resolves the interned name back to a string.
    pub landwalk: Vec<&'static str>,
}

impl Expected {
    /// Derive the fingerprint from a manifest row. Mirrors
    /// `scryfall`'s type-line splitting so a subtype that shares a
    /// type's text can't false-positive.
    pub fn from_row(row: &DumpRow) -> Self {
        let tp = type_part(&row.type_line);
        let has = |t: &str| tp.contains(t);

        let color = |c: &str| row.colors.iter().any(|x| x == c);

        // Mana value: Scryfall `cmc` is an integer for everything
        // T1/T2 touches (no X costs reach this path). A card with
        // no mana cost (land) must have `mana_cost: None`.
        let mana_value = if row.mana_cost.is_some() {
            Some(row.cmc.round() as u32)
        } else {
            None
        };

        Self {
            name: row.name.clone(),
            mana_value,
            colors: [
                color("W"),
                color("U"),
                color("B"),
                color("R"),
                color("G"),
            ],
            is_creature: has("Creature"),
            is_instant: has("Instant"),
            is_sorcery: has("Sorcery"),
            is_artifact: has("Artifact"),
            is_enchantment: has("Enchantment"),
            is_land: has("Land"),
            is_planeswalker: has("Planeswalker"),
            power: pt_expectation(row.is_creature_row(), &row.power),
            toughness: pt_expectation(row.is_creature_row(), &row.toughness),
            keywords: row
                .keywords
                .iter()
                .filter_map(|k| evergreen_variant(k))
                .collect(),
            landwalk: row
                .keywords
                .iter()
                .filter_map(|k| landwalk_subtype(k))
                .collect(),
        }
    }
}

impl DumpRow {
    fn is_creature_row(&self) -> bool {
        type_part(&self.type_line).contains("Creature")
    }
}

/// `Some(Some(n))` for an integer P/T on a creature, `Some(None)`
/// for an absent P/T on a creature (rare but valid), `None` to skip
/// the assertion (non-creature, or `*`/`1+*`-style values we can't
/// pin to a `Fixed`).
fn pt_expectation(is_creature: bool, raw: &Option<String>) -> Option<Option<i32>> {
    if !is_creature {
        return None;
    }
    match raw {
        None => Some(None),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => Some(Some(n)),
            Err(_) => None, // "*", "1+*", etc. — not robustly checkable
        },
    }
}

/// Map a Scryfall keyword string to the matching `KeywordAbility`
/// unit-variant ident, or `None` if it's not an evergreen unit
/// variant (parametrised keywords like Ward/Equip carry data we
/// can't reconstruct from the manifest, so we don't assert them).
fn evergreen_variant(scryfall_kw: &str) -> Option<&'static str> {
    Some(match scryfall_kw.to_lowercase().as_str() {
        "flying" => "Flying",
        "first strike" => "FirstStrike",
        "double strike" => "DoubleStrike",
        "deathtouch" => "Deathtouch",
        "defender" => "Defender",
        "haste" => "Haste",
        "hexproof" => "Hexproof",
        "indestructible" => "Indestructible",
        "lifelink" => "Lifelink",
        "menace" => "Menace",
        "reach" => "Reach",
        "trample" => "Trample",
        "vigilance" => "Vigilance",
        "flash" => "Flash",
        "shroud" => "Shroud",
        // Pass 2 — static per-pairing evasion, fully enforced.
        "fear" => "Fear",
        "intimidate" => "Intimidate",
        "shadow" => "Shadow",
        "horsemanship" => "Horsemanship",
        "skulk" => "Skulk",
        _ => return None,
    })
}

/// Map a Scryfall landwalk keyword to the basic land-subtype name it
/// references. `KeywordAbility::Landwalk` carries the interned subtype
/// (`"Forest"`…), so L2 asserts that exact type. The generic
/// `"Landwalk"` umbrella Scryfall also emits is intentionally dropped
/// (returns `None`) — the specific `"<type>walk"` entry certifies it.
fn landwalk_subtype(scryfall_kw: &str) -> Option<&'static str> {
    Some(match scryfall_kw.to_lowercase().as_str() {
        "plainswalk" => "Plains",
        "islandwalk" => "Island",
        "swampwalk" => "Swamp",
        "mountainwalk" => "Mountain",
        "forestwalk" => "Forest",
        _ => return None,
    })
}

/// Render the `#[cfg(test)]` structural-assertion module to append
/// to a candidate's source. Self-contained: uses only `arcana_core`
/// and `std`. Collects every mismatch and fails once with the full
/// list, so a `cargo test` failure names exactly what diverged.
pub fn render_harness(exp: &Expected) -> String {
    let mut a = String::new();

    // name — bind the expected string as a local so the Debug-
    // quoted literal is only substituted once (into a `let`), never
    // embedded inside another string literal.
    a.push_str(&format!(
        "        let want_name = {n:?};\n\
         \x20       match reg.interner().resolve(def.name) {{\n\
         \x20           Some(s) if s == want_name => {{}}\n\
         \x20           Some(other) => bad.push(format!(\"name: got {{other:?}}, want {{want_name:?}}\")),\n\
         \x20           None => bad.push(\"name: interner could not resolve def.name\".into()),\n\
         \x20       }}\n",
        n = exp.name,
    ));

    // mana value
    match exp.mana_value {
        Some(mv) => a.push_str(&format!(
            "        match def.base_characteristics.mana_cost.as_ref().map(|m| m.mana_value()) {{\n\
             \x20           Some({mv}) => {{}}\n\
             \x20           got => bad.push(format!(\"mana_value: got {{got:?}}, want Some({mv})\")),\n\
             \x20       }}\n",
        )),
        None => a.push_str(
            "        if def.base_characteristics.mana_cost.is_some() {\n\
             \x20           bad.push(\"mana_cost: got Some(_), want None (no mana cost)\".into());\n\
             \x20       }\n"
                .into(),
        ),
    }

    // colors
    for (i, (col, want)) in
        ["White", "Blue", "Black", "Red", "Green"].iter().zip(exp.colors).enumerate()
    {
        let _ = i;
        a.push_str(&format!(
            "        if def.base_characteristics.colors.contains(Color::{col}) != {want} {{\n\
             \x20           bad.push(format!(\"color {col}: got {{}}, want {want}\", \
             def.base_characteristics.colors.contains(Color::{col})));\n\
             \x20       }}\n",
        ));
    }

    // type flags
    for (method, want) in [
        ("is_creature", exp.is_creature),
        ("is_instant", exp.is_instant),
        ("is_sorcery", exp.is_sorcery),
        ("is_artifact", exp.is_artifact),
        ("is_enchantment", exp.is_enchantment),
        ("is_land", exp.is_land),
        ("is_planeswalker", exp.is_planeswalker),
    ] {
        a.push_str(&format!(
            "        if def.base_characteristics.types.{method}() != {want} {{\n\
             \x20           bad.push(format!(\"{method}: got {{}}, want {want}\", \
             def.base_characteristics.types.{method}()));\n\
             \x20       }}\n",
        ));
    }

    // power / toughness
    a.push_str(&pt_assertion("power", exp.power));
    a.push_str(&pt_assertion("toughness", exp.toughness));

    // keywords (presence only — see module docs)
    for kw in &exp.keywords {
        a.push_str(&format!(
            "        if !def.base_characteristics.keywords.iter()\n\
             \x20           .any(|k| matches!(k, KeywordAbility::{kw})) {{\n\
             \x20           bad.push(\"keyword: missing KeywordAbility::{kw}\".into());\n\
             \x20       }}\n",
        ));
    }

    // landwalk — parametrised: assert a Landwalk whose interned
    // subtype resolves back to the expected basic land-type name.
    for lt in &exp.landwalk {
        a.push_str(&format!(
            "        if !def.base_characteristics.keywords.iter()\n\
             \x20           .any(|k| matches!(k, KeywordAbility::Landwalk(s)\n\
             \x20               if reg.interner().resolve(*s) == Some({lt:?}))) {{\n\
             \x20           bad.push(\"keyword: missing KeywordAbility::Landwalk({lt})\".into());\n\
             \x20       }}\n",
        ));
    }

    format!(
        "\n\
        #[cfg(test)]\n\
        #[allow(unused_imports)]\n\
        mod __structural {{\n\
        \x20   //! Codegen'd by arcana-gen::structural — layer-2 fingerprint\n\
        \x20   //! diff against the Scryfall row this card was generated from.\n\
        \x20   use arcana_core::registry::CardRegistry;\n\
        \x20   use arcana_core::types::{{Color, PtValue}};\n\
        \x20   use arcana_core::effects::KeywordAbility;\n\
        \n\
        \x20   #[test]\n\
        \x20   fn structural() {{\n\
        \x20       let mut reg = CardRegistry::new();\n\
        \x20       let id = super::register(&mut reg);\n\
        \x20       let def = reg.get(id).expect(\"register returned an unknown CardId\");\n\
        \x20       let mut bad: Vec<String> = Vec::new();\n\
        {a}\
        \x20       assert!(bad.is_empty(), \"structural mismatches:\\n  {{}}\", bad.join(\"\\n  \"));\n\
        \x20   }}\n\
        }}\n",
    )
}

fn pt_assertion(field: &str, exp: Option<Option<i32>>) -> String {
    match exp {
        None => String::new(), // skipped (non-creature or `*`)
        Some(Some(n)) => format!(
            "        match def.base_characteristics.{field} {{\n\
             \x20           Some(PtValue::Fixed({n})) => {{}}\n\
             \x20           got => bad.push(format!(\"{field}: got {{got:?}}, want Some(Fixed({n}))\")),\n\
             \x20       }}\n",
        ),
        Some(None) => format!(
            "        if def.base_characteristics.{field}.is_some() {{\n\
             \x20           bad.push(\"{field}: got Some(_), want None\".into());\n\
             \x20       }}\n",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(configure: impl FnOnce(&mut DumpRow)) -> DumpRow {
        let mut r = DumpRow {
            idx: 0,
            slug: "x".into(),
            tier: 1,
            supported: true,
            shape: Some("VanillaCreature".into()),
            unsupported_reason: None,
            name: "Grizzly Bears".into(),
            oracle_id: "o".into(),
            set: "lea".into(),
            mana_cost: Some("{1}{G}".into()),
            cmc: 2.0,
            type_line: "Creature — Bear".into(),
            power: Some("2".into()),
            toughness: Some("2".into()),
            colors: vec!["G".into()],
            keywords: vec![],
            prompt_file: Some("prompts/000_x.txt".into()),
        };
        configure(&mut r);
        r
    }

    #[test]
    fn vanilla_creature_fingerprint() {
        let e = Expected::from_row(&row(|_| {}));
        assert_eq!(e.name, "Grizzly Bears");
        assert_eq!(e.mana_value, Some(2));
        assert_eq!(e.colors, [false, false, false, false, true]);
        assert!(e.is_creature);
        assert!(!e.is_instant && !e.is_land);
        assert_eq!(e.power, Some(Some(2)));
        assert_eq!(e.toughness, Some(Some(2)));
        assert!(e.keywords.is_empty());
    }

    #[test]
    fn instant_has_no_pt_assertion() {
        let e = Expected::from_row(&row(|r| {
            r.name = "Lightning Bolt".into();
            r.type_line = "Instant".into();
            r.mana_cost = Some("{R}".into());
            r.cmc = 1.0;
            r.colors = vec!["R".into()];
            r.power = None;
            r.toughness = None;
        }));
        assert!(e.is_instant && !e.is_creature);
        assert_eq!(e.power, None, "non-creature P/T must not be asserted");
        assert_eq!(e.mana_value, Some(1));
    }

    #[test]
    fn star_power_is_skipped() {
        let e = Expected::from_row(&row(|r| {
            r.power = Some("*".into());
            r.toughness = Some("*".into());
        }));
        assert_eq!(e.power, None, "`*` P/T is not robustly checkable");
    }

    #[test]
    fn land_expects_no_mana_cost() {
        let e = Expected::from_row(&row(|r| {
            r.name = "Forest".into();
            r.type_line = "Basic Land — Forest".into();
            r.mana_cost = None;
            r.cmc = 0.0;
            r.colors = vec![];
            r.power = None;
            r.toughness = None;
        }));
        assert_eq!(e.mana_value, None);
        assert!(e.is_land && !e.is_creature);
    }

    #[test]
    fn french_vanilla_keywords_mapped() {
        let e = Expected::from_row(&row(|r| {
            r.name = "Serra Angel".into();
            r.type_line = "Creature — Angel".into();
            r.keywords = vec!["Flying".into(), "Vigilance".into(), "Ward".into()];
        }));
        // Flying + Vigilance map; Ward is parametrised → dropped.
        assert_eq!(e.keywords, vec!["Flying", "Vigilance"]);
    }

    #[test]
    fn harness_is_self_contained_and_names_fields() {
        let h = render_harness(&Expected::from_row(&row(|_| {})));
        assert!(h.contains("#[cfg(test)]"));
        assert!(h.contains("mod __structural"));
        assert!(h.contains("super::register"));
        assert!(h.contains("use arcana_core::registry::CardRegistry;"));
        // No arcana-gen / arcana-cards references — must compile
        // inside arcana-cards.
        assert!(!h.contains("arcana_gen"));
        assert!(!h.contains("arcana_cards"));
    }
}

