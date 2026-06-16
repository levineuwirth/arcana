//! Tamiyo, Compleated Sage — `{2}{G}{G/U/P}{U}` legendary planeswalker,
//! starting loyalty 3 (printed). Colors G, U. Subtype Tamiyo.
//!
//! Oracle text:
//! * Compleated ({G/U/P} can be paid with {G}, {U}, or 2 life; if life
//!   was paid this planeswalker enters with two fewer loyalty counters.)
//! * `+1`: Tap up to one target artifact or creature. It doesn't untap
//!   during its controller's next untap step.
//! * `−X`: Exile target nonland permanent card with mana value X from
//!   your graveyard. Create a token that's a copy of that card.
//! * `−7`: Create Tamiyo's Notebook, a legendary colorless Book artifact
//!   token with "Spells you cast cost {2} less to cast" and
//!   "{T}: Draw a card."
//!
//! # Rules references
//!
//! * CR 113.3c — entering with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities (cost = add/remove loyalty counters).
//! * CR 606.3 — controller-only, sorcery speed, stack empty, once/turn.
//! * CR 704.5i — 0-loyalty state-based sacrifice.
//!
//! # Scope / GAPs
//!
//! * Compleated is NOT in the usable keyword surface — `keywords: vec![]`
//!   and the "pay 2 life for two fewer loyalty" rider is GAP'd.
//! * `+1`: the Tap IS implemented (`Effect::Tap`). The "doesn't untap
//!   during its controller's next untap step" rider has no primitive in
//!   the demonstrated surface — GAP'd (Tap applied, no-untap omitted).
//! * `−X`: the dynamic-X loyalty COST is expressible
//!   (`remove_loyalty_x: true`), but the body GAPs: targeting a card in
//!   *your graveyard* needs a concrete `Zone::Graveyard(player)` (no
//!   any-graveyard sentinel) and "create a token that's a copy of that
//!   card" is not expressible. Shell declared, body returns `Vec::new()`.
//! * `−7`: the Notebook token SHELL is created, but its two granted
//!   abilities — "Spells you cast cost {2} less" (a static) and
//!   "{T}: Draw a card" (an activated ability) — are NOT expressible via
//!   `TokenDefinition` (its `abilities` field is `Vec<TriggeredAbilityDef>`).
//!   Both GAP'd. Legendary supertype is not representable on the
//!   demonstrated `TokenDefinition` (no supertypes field) — omitted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tamiyo, Compleated Sage");
    let tamiyo = reg.interner_mut().intern("Tamiyo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tamiyo);

    // Token shell symbols for the −7 ultimate (interned now so the
    // resolver can look them up read-only).
    let _ = reg.interner_mut().intern("Tamiyo's Notebook");
    let _ = reg.interner_mut().intern("Book");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G/U/P}{U}").expect("valid cost")),
        // Colors per the spec's `Colors:` line (G, U). The {G/U/P}
        // Phyrexian hybrid is colored G/U.
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // Printed starting loyalty 3 (CR 113.3c).
        loyalty: Some(3),
        // GAP: Compleated is not in the usable keyword surface; the
        // "pay 2 life → enters with two fewer loyalty" rider is unmodeled.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Tap up to one target artifact or creature. It \
                       doesn't untap during its controller's next untap \
                       step.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::CREATURE,
                        )),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_tap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−X: Exile target nonland permanent card with mana \
                       value X from your graveyard. Create a token that's a \
                       copy of that card.".into(),
                cost: ActivationCost {
                    remove_loyalty_x: true,
                    ..ActivationCost::default()
                },
                // GAP: targeting a card in YOUR graveyard requires a
                // concrete Zone::Graveyard(player) target (no any-graveyard
                // sentinel in the demonstrated surface), so no target
                // requirement is declared here.
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_x_exile_copy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Create Tamiyo's Notebook, a legendary colorless \
                       Book artifact token with \"Spells you cast cost {2} \
                       less to cast\" and \"{T}: Draw a card.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ultimate_notebook,
            }),
    )
}

/// `+1: Tap up to one target artifact or creature.`
///
/// The Tap is applied. The "doesn't untap during its controller's next
/// untap step" rider has no primitive in the demonstrated surface — GAP'd.
fn plus_one_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "up to one" — no target chosen is legal.
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Tap { target: *id }]
    // GAP: no "doesn't untap during controller's next untap step" primitive.
}

/// `−X: Exile target nonland permanent card with mana value X from your
/// graveyard. Create a token that's a copy of that card.`
fn minus_x_exile_copy(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the dynamic-X loyalty COST is expressible (remove_loyalty_x),
    // but the effect body is not. Targeting a card in YOUR graveyard needs
    // a concrete Zone::Graveyard(player) target (no any-graveyard
    // sentinel), and "create a token that's a copy of that card" is not
    // expressible in the demonstrated Effect surface. X is read via
    // ctx.x_value.unwrap_or(0) once a graveyard-card target and a
    // copy-token primitive exist.
    Vec::new()
}

/// `−7: Create Tamiyo's Notebook.`
///
/// Creates the artifact token SHELL. Its two granted abilities — the
/// "{2} less to cast" static and the "{T}: Draw a card" activated ability
/// — are not expressible via `TokenDefinition`, so both are GAP'd, as is
/// the legendary supertype (no supertypes field on `TokenDefinition`).
fn ultimate_notebook(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Both symbols were interned in register(); look them up read-only.
    let book = reg.interner().lookup("Book");
    let mut subtypes = SubtypeSet::default();
    if let Some(sym) = book {
        subtypes.0.insert(sym);
    }
    let Some(name) = reg.interner().lookup("Tamiyo's Notebook") else {
        return Vec::new();
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            // GAP: "Spells you cast cost {2} less" (static) and
            // "{T}: Draw a card" (activated) aren't expressible here —
            // TokenDefinition.abilities is Vec<TriggeredAbilityDef> only.
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
