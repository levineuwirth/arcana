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
//!
//! For T3 triggered cards it adds a *structural* trigger check: a
//! card whose oracle text introduces a triggered ability must
//! register at least that many `TriggeredAbilityDef`s, and — for
//! oracle phrasings that map unambiguously to one `TriggerCondition`
//! variant — at least one ability must carry that variant. This
//! closes the dominant T3 failure (the trigger silently dropped, the
//! card generated as a vanilla creature). It still does not verify
//! the trigger's *effect* — that stays a job for the semantic audit.

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
    /// Lower bound on the card's `triggered_abilities` count, derived
    /// from the number of trigger-introducing oracle lines. `0` for
    /// cards with no triggered ability — no assertion is emitted.
    /// Asserted as `>=` (never `==`): keyword-synthesised triggers can
    /// only add to the count, so an exact check would false-fail.
    pub min_triggered_abilities: usize,
    /// `TriggerCondition` variant idents at least one registered
    /// triggered ability must match. Populated only for oracle
    /// phrasings that map unambiguously to a single variant
    /// (self-dies, self-enters, "whenever you cast", upkeep). Ambiguous
    /// phrasings (bare "attacks", "a creature enters") are dropped —
    /// same discipline as parametrised keywords — and rely on the
    /// `min_triggered_abilities` presence check alone.
    pub trigger_kinds: Vec<&'static str>,
}

impl Expected {
    /// Derive the fingerprint from a manifest row. Mirrors
    /// `scryfall`'s type-line splitting so a subtype that shares a
    /// type's text can't false-positive.
    ///
    /// `oracle` is the card's authoritative Scryfall oracle text (the
    /// caller pulls it from the dumped prompt). An empty `oracle`
    /// degrades cleanly to "no trigger assertions" — never a false
    /// quarantine — matching the dynamic-literal gate's policy.
    pub fn from_row(row: &DumpRow, oracle: &str) -> Self {
        let tp = type_part(&row.type_line);
        let has = |t: &str| tp.contains(t);
        // For multi-face shapes whose registered base is the front
        // face (Adventure / MDFC), the generated card implements only
        // the front face — the back/adventure half is an
        // `AlternateFace` (or GAP'd engine debt). The joined oracle
        // carries BOTH faces' text (separated by `\n---\n`), so
        // deriving trigger expectations from the whole thing would
        // demand triggers the front-face card legitimately doesn't
        // carry. Scope the trigger derivation to the front face.
        let trig_source = front_face_oracle(row.shape.as_deref(), oracle);
        let trig_lines = trigger_lines(&trig_source);

        // Faces-only front-base shapes (Adventure / MDFC / Transform /
        // Battle): Scryfall's top-level `keywords` is the COMBINED set
        // across both faces, but the generated card implements only the
        // front face. When the front face isn't the keyword-bearer
        // (e.g. a Battle front whose keywords all live on the back-face
        // creature), asserting the combined keywords false-quarantines a
        // correct card. The manifest can't attribute a keyword to a
        // face, so — consistent with the front-scoped trigger
        // derivation above — drop the keyword/landwalk assertion for
        // these shapes (degrade to "no keyword assertion", never a false
        // fail).
        let faces_only = matches!(
            row.shape.as_deref(),
            Some("AdventureCreature") | Some("ModalDfcCreature")
            | Some("TransformCreature") | Some("Battle"),
        );

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
            name: expected_base_name(row),
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
            keywords: if faces_only {
                Vec::new()
            } else {
                row.keywords.iter().filter_map(|k| evergreen_variant(k)).collect()
            },
            landwalk: if faces_only {
                Vec::new()
            } else {
                row.keywords.iter().filter_map(|k| landwalk_subtype(k)).collect()
            },
            trigger_kinds: confident_trigger_kinds(&trig_lines, &row.name),
            min_triggered_abilities: trig_lines.len(),
        }
    }
}

impl DumpRow {
    fn is_creature_row(&self) -> bool {
        type_part(&self.type_line).contains("Creature")
    }
}

/// The portion of the oracle text the registered base face is
/// responsible for. For Adventure / MDFC the joined oracle carries
/// both faces separated by `\n---\n` (see
/// `Card::effective_oracle_text`); the generated card implements only
/// the front face, so trigger expectations must be derived from the
/// front-face text alone. Single-face shapes return the oracle
/// unchanged.
fn front_face_oracle(shape: Option<&str>, oracle: &str) -> String {
    let is_front_face_base = matches!(
        shape,
        Some("AdventureCreature") | Some("ModalDfcCreature")
        | Some("TransformCreature") | Some("Battle"),
    );
    if is_front_face_base {
        if let Some((front, _back)) = oracle.split_once("\n---\n") {
            return front.to_string();
        }
    }
    oracle.to_string()
}

/// The base-characteristics name the engine registers for this card,
/// which is what the layer-2 harness checks against `def.name`.
///
/// Most cards register their full printed name. Multi-face cards whose
/// *front* face is the registered base (Adventure and MDFC — the card
/// is the front-face permanent, the back/adventure half lives in an
/// `AlternateFace`) register only the front-face name. Scryfall's
/// `name` for those is the combined `"Front // Back"`, so the
/// assertion must compare against the front-face portion or it
/// false-fails on every such card.
///
/// Split cards (combined name registered via
/// `combine_split_characteristics`) are Tier-4 / out of scope and
/// never reach this harness, so they need no special case here.
fn expected_base_name(row: &DumpRow) -> String {
    let is_front_face_base = matches!(
        row.shape.as_deref(),
        Some("AdventureCreature") | Some("ModalDfcCreature")
        | Some("TransformCreature") | Some("Battle"),
    );
    if is_front_face_base {
        if let Some((front, _back)) = row.name.split_once(" // ") {
            return front.to_string();
        }
    }
    row.name.clone()
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
        // Pass 3.1 — damage-as-counters, fully enforced. (`Toxic` is
        // parametrised `Toxic(u8)`; like Ward its N isn't in the
        // Scryfall keyword list, so it's dropped — not asserted.)
        "wither" => "Wither",
        "infect" => "Infect",
        // Pass 3.2 — death-triggered, fully enforced. (`Afterlife` is
        // parametrised `Afterlife(u32)`; its N is in rules text, not
        // the Scryfall keyword list, so dropped like Toxic/Ward.)
        "undying" => "Undying",
        "persist" => "Persist",
        // Pass 3.3 — attack/combat-damage triggered, fully enforced.
        // (`Renown` is parametrised `Renown(u32)`; N is in rules
        // text, so dropped like Toxic/Afterlife.)
        "exalted" => "Exalted",
        "battle cry" => "BattleCry",
        "mentor" => "Mentor",
        "dethrone" => "Dethrone",
        "enlist" => "Enlist",
        // Pass 3.4 — block-time combat statics, fully enforced.
        // (`Rampage`/`Bushido` are parametrised; N is in rules text,
        // dropped like Toxic/Renown.)
        "flanking" => "Flanking",
        "provoke" => "Provoke",
        // Pass 3.5 — ETB scaling, fully enforced. (Modular/Graft/
        // Bloodthirst/Devour/Amplify are parametrised; N is in rules
        // text, dropped like Toxic/Renown/Rampage.)
        "sunburst" => "Sunburst",
        "unleash" => "Unleash",
        "riot" => "Riot",
        // Pass 3.6 — long tail, fully implemented or recognized with
        // a documented Phase-1 policy. (Fading/Vanishing are
        // parametrised; N is in rules text, dropped like Toxic.
        // Banding stays deferred — not asserted, cards quarantined.)
        "evolve" => "Evolve",
        // `soulshift` is parametrised `Soulshift(u8)` (N in rules
        // text) → dropped like Toxic/Renown, asserted by neither.
        // `scavenge` is parametrised `Scavenge(ManaCost)` (cost in
        // rules text) → dropped like Cycling/Ward, asserted by neither
        // (Pass 4.4b made it a real graveyard activated ability).
        "changeling" => "Changeling",
        // Pass 3.8 — Banding recognized (choice-control; Phase-1
        // inert, documented DEBT). Asserted like any unit keyword.
        "banding" => "Banding",
        // Pass 3.7 — Warp is a deferred marker: assert it so a card
        // carrying it must emit `KeywordAbility::Warp` and the L3
        // honesty guard quarantines it (Ward/Cycling are
        // parametrised → dropped like Toxic, asserted by neither).
        "warp" => "Warp",
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

/// Strip `(reminder text)` from an oracle string. A local copy of
/// `classifier::strip_reminder_text` — keeping this module standalone
/// matters more than de-duplicating ten lines.
fn strip_reminder(s: &str) -> String {
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

/// The lower-cased oracle lines that introduce a triggered ability.
/// Mirrors `classifier::has_triggered_ability`'s per-line scan: MTG
/// separates ability paragraphs with `\n`, and a trigger word only
/// counts at the start of its line (so "destroy ~ when ..." riders
/// and reminder text don't false-positive).
fn trigger_lines(oracle: &str) -> Vec<String> {
    let stripped = strip_reminder(oracle);
    stripped
        .lines()
        .map(|l| l.trim_start().to_lowercase())
        .filter(|l| {
            ["when ", "whenever ", "at the beginning", "at the end"]
                .iter()
                .any(|t| l.starts_with(t))
        })
        .collect()
}

/// Map trigger lines to `TriggerCondition` variant idents, keeping
/// only the phrasings that pin to exactly one variant. `card_name`
/// disambiguates a *self* trigger ("When <name> enters") from a
/// filtered one ("Whenever a creature enters"), which the engine
/// models as `SelfEntersBattlefield` vs `ZoneChange` respectively.
/// Anything ambiguous (bare "attacks" — `SelfAttacks` vs
/// `CreatureAttacks`; "end step" — `StepBegins` vs `PhaseBegins`) is
/// dropped: presence is still enforced via `min_triggered_abilities`.
fn confident_trigger_kinds(lines: &[String], card_name: &str) -> Vec<&'static str> {
    let name = card_name.to_lowercase();
    let mut kinds: Vec<&'static str> = Vec::new();
    let add = |k: &'static str, kinds: &mut Vec<&'static str>| {
        if !kinds.contains(&k) {
            kinds.push(k);
        }
    };
    for line in lines {
        let self_verb = |verb: &str| {
            line.contains(&format!("{name} {verb}"))
                || line.contains(&format!("this creature {verb}"))
                || line.contains(&format!("this permanent {verb}"))
        };
        if self_verb("dies") {
            add("SelfDies", &mut kinds);
        }
        if self_verb("enters") {
            add("SelfEntersBattlefield", &mut kinds);
        }
        if line.contains("whenever you cast")
            || line.contains("whenever an opponent casts")
            || line.contains("whenever a player casts")
        {
            add("SpellCast", &mut kinds);
        }
        if line.starts_with("at the beginning of") && line.contains("upkeep") {
            add("StepBegins", &mut kinds);
        }
    }
    kinds
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

    // triggered abilities — presence (the trigger wasn't silently
    // dropped) plus a confident-kind check. See module docs: this is
    // structural, not a proof the trigger's effect is right.
    if exp.min_triggered_abilities > 0 {
        let m = exp.min_triggered_abilities;
        a.push_str(&format!(
            "        if def.triggered_abilities.len() < {m} {{\n\
             \x20           bad.push(format!(\"triggered abilities: got {{}}, want >= {m}\", \
             def.triggered_abilities.len()));\n\
             \x20       }}\n",
        ));
    }
    for kind in &exp.trigger_kinds {
        let pat = match *kind {
            "SelfDies" => "TriggerCondition::SelfDies",
            "SelfEntersBattlefield" => "TriggerCondition::SelfEntersBattlefield",
            "SpellCast" => "TriggerCondition::SpellCast { .. }",
            "StepBegins" => "TriggerCondition::StepBegins { .. }",
            _ => continue,
        };
        a.push_str(&format!(
            "        if !def.triggered_abilities.iter()\n\
             \x20           .any(|t| matches!(t.trigger_condition, {pat})) {{\n\
             \x20           bad.push(\"trigger: no registered ability with condition {kind}\".into());\n\
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
        \x20   use arcana_core::triggers::TriggerCondition;\n\
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
        let e = Expected::from_row(&row(|_| {}), "");
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
    fn adventure_expects_front_face_name_not_combined() {
        // Scryfall reports the combined "Front // Back" name, but the
        // engine registers the front-face creature name as the base.
        // The harness must expect the front-face portion or every
        // Adventure card false-fails its name assertion.
        let e = Expected::from_row(&row(|r| {
            r.shape = Some("AdventureCreature".into());
            r.name = "Garenbrig Carver // Shield's Might".into();
            r.type_line = "Creature — Human Warrior // Instant — Adventure".into();
            r.mana_cost = Some("{3}{G}".into());
            r.cmc = 4.0;
            r.power = Some("3".into());
            r.toughness = Some("2".into());
        }), "");
        assert_eq!(e.name, "Garenbrig Carver",
            "Adventure name assertion must target the front face");
        assert!(e.is_creature, "Adventure base face is the creature");
    }

    #[test]
    fn mdfc_expects_front_face_name_not_combined() {
        let e = Expected::from_row(&row(|r| {
            r.shape = Some("ModalDfcCreature".into());
            r.name = "Jwari Disruption // Jwari Ruins".into();
            r.type_line = "Instant // Land".into();
        }), "");
        assert_eq!(e.name, "Jwari Disruption",
            "MDFC name assertion must target the front face");
    }

    #[test]
    fn transform_expects_front_face_name_not_combined() {
        // CR 712 transform DFC: front creature is the registered base;
        // the back face is reached only by transforming. The name
        // assertion (and trigger derivation) must scope to the front.
        let e = Expected::from_row(&row(|r| {
            r.shape = Some("TransformCreature".into());
            r.name = "Mayor of Avabruck // Howlpack Alpha".into();
            r.type_line = "Creature — Human Advisor Werewolf // Creature — Werewolf".into();
            r.mana_cost = Some("{1}{G}".into());
            r.cmc = 2.0;
            r.power = Some("1".into());
            r.toughness = Some("1".into());
        }), "");
        assert_eq!(e.name, "Mayor of Avabruck",
            "Transform name assertion must target the front face");
        assert!(e.is_creature, "Transform base face is the creature");
    }

    #[test]
    fn battle_expects_front_face_name_and_drops_combined_keywords() {
        // MOM Sieges are faces-only DFCs: front is a Battle, back is a
        // creature/planeswalker. Scryfall's combined name + combined
        // keyword list both describe the pair. The front-base card
        // registers only the front Battle (front name, no keywords —
        // its keywords live on the back creature). Verify must scope
        // the name to the front and NOT assert the back's keywords.
        let e = Expected::from_row(&row(|r| {
            r.shape = Some("Battle".into());
            r.name = "Invasion of Dominaria // Serra Faithkeeper".into();
            r.type_line = "Battle — Siege // Creature — Angel".into();
            r.mana_cost = Some("{2}{W}".into());
            r.cmc = 3.0;
            r.colors = vec!["W".into()];
            r.keywords = vec!["Flying".into(), "Vigilance".into()];
            r.power = None;
            r.toughness = None;
        }), "");
        assert_eq!(e.name, "Invasion of Dominaria",
            "Battle name assertion targets the front face");
        assert!(!e.is_creature, "the front Battle is not a creature");
        assert!(e.keywords.is_empty(),
            "combined back-face keywords are NOT asserted on the front Battle");
    }

    #[test]
    fn single_face_name_is_unchanged() {
        // A normal card with no " // " keeps its full name even if it
        // somehow carried a multi-face shape tag.
        let e = Expected::from_row(&row(|r| {
            r.name = "Grizzly Bears".into();
        }), "");
        assert_eq!(e.name, "Grizzly Bears");
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
        }), "");
        assert!(e.is_instant && !e.is_creature);
        assert_eq!(e.power, None, "non-creature P/T must not be asserted");
        assert_eq!(e.mana_value, Some(1));
    }

    #[test]
    fn star_power_is_skipped() {
        let e = Expected::from_row(&row(|r| {
            r.power = Some("*".into());
            r.toughness = Some("*".into());
        }), "");
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
        }), "");
        assert_eq!(e.mana_value, None);
        assert!(e.is_land && !e.is_creature);
    }

    #[test]
    fn french_vanilla_keywords_mapped() {
        let e = Expected::from_row(&row(|r| {
            r.name = "Serra Angel".into();
            r.type_line = "Creature — Angel".into();
            r.keywords = vec!["Flying".into(), "Vigilance".into(), "Ward".into()];
        }), "");
        // Flying + Vigilance map; Ward is parametrised → dropped.
        assert_eq!(e.keywords, vec!["Flying", "Vigilance"]);
    }

    #[test]
    fn harness_is_self_contained_and_names_fields() {
        let h = render_harness(&Expected::from_row(&row(|_| {}), ""));
        assert!(h.contains("#[cfg(test)]"));
        assert!(h.contains("mod __structural"));
        assert!(h.contains("super::register"));
        assert!(h.contains("use arcana_core::registry::CardRegistry;"));
        // No arcana-gen / arcana-cards references — must compile
        // inside arcana-cards.
        assert!(!h.contains("arcana_gen"));
        assert!(!h.contains("arcana_cards"));
    }

    // --- trigger fingerprint --------------------------------------

    #[test]
    fn etb_trigger_fingerprint() {
        let e = Expected::from_row(
            &row(|r| {
                r.name = "Elvish Visionary".into();
                r.type_line = "Creature — Elf Shaman".into();
            }),
            "When Elvish Visionary enters, draw a card.",
        );
        assert_eq!(e.min_triggered_abilities, 1);
        assert_eq!(e.trigger_kinds, vec!["SelfEntersBattlefield"]);
    }

    #[test]
    fn dies_trigger_fingerprint() {
        let e = Expected::from_row(
            &row(|r| r.name = "Solemn Simulacrum".into()),
            "When Solemn Simulacrum dies, draw a card.",
        );
        assert_eq!(e.min_triggered_abilities, 1);
        assert_eq!(e.trigger_kinds, vec!["SelfDies"]);
    }

    #[test]
    fn cast_trigger_fingerprint() {
        let e = Expected::from_row(
            &row(|r| r.name = "Young Pyromancer".into()),
            "Whenever you cast an instant or sorcery spell, create a 1/1 red \
             Elemental creature token.",
        );
        assert_eq!(e.min_triggered_abilities, 1);
        assert_eq!(e.trigger_kinds, vec!["SpellCast"]);
    }

    #[test]
    fn upkeep_trigger_fingerprint() {
        let e = Expected::from_row(
            &row(|r| r.name = "Howling Mine".into()),
            "At the beginning of each player's upkeep, that player draws a card.",
        );
        assert_eq!(e.min_triggered_abilities, 1);
        assert_eq!(e.trigger_kinds, vec!["StepBegins"]);
    }

    #[test]
    fn ambiguous_trigger_kind_dropped_presence_kept() {
        // "attacks" maps to SelfAttacks *or* CreatureAttacks{filter} —
        // ambiguous, so no kind is asserted; presence still is.
        let e = Expected::from_row(
            &row(|r| r.name = "Rampaging Brontodon".into()),
            "Whenever Rampaging Brontodon attacks, it gets +1/+0 until end of turn.",
        );
        assert_eq!(e.min_triggered_abilities, 1);
        assert!(e.trigger_kinds.is_empty());
    }

    #[test]
    fn enters_tapped_is_not_a_trigger_line() {
        // "~ enters tapped" is a replacement effect, not a triggered
        // ability: the line doesn't start with a trigger word, so it
        // must not inflate the count or assert SelfEntersBattlefield.
        let e = Expected::from_row(
            &row(|r| r.name = "Drowned Catacomb".into()),
            "Drowned Catacomb enters tapped unless you control an Island or a Swamp.",
        );
        assert_eq!(e.min_triggered_abilities, 0);
        assert!(e.trigger_kinds.is_empty());
    }

    #[test]
    fn vanilla_card_has_no_trigger_fingerprint() {
        let e = Expected::from_row(&row(|_| {}), "");
        assert_eq!(e.min_triggered_abilities, 0);
        assert!(e.trigger_kinds.is_empty());
    }

    #[test]
    fn harness_emits_trigger_assertions_for_etb() {
        let h = render_harness(&Expected::from_row(
            &row(|r| r.name = "Elvish Visionary".into()),
            "When Elvish Visionary enters, draw a card.",
        ));
        assert!(h.contains("def.triggered_abilities.len() < 1"));
        assert!(h.contains("TriggerCondition::SelfEntersBattlefield"));
        assert!(h.contains("use arcana_core::triggers::TriggerCondition;"));
    }
}

