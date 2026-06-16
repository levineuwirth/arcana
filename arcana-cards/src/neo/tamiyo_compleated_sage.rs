//! Tamiyo, Compleated Sage — `{2}{G}{G/U/P}{U}` Legendary Planeswalker — Tamiyo.
//! Colors G, U. Starting loyalty 3. Keyword: Compleated.
//!
//! +1: Tap up to one target artifact or creature. It doesn't untap during its
//!   controller's next untap step.
//! −X: Exile target nonland permanent card with mana value X from your
//!   graveyard. Create a token that's a copy of that card.
//! −7: Create Tamiyo's Notebook, a legendary colorless Book artifact token with
//!   "Spells you cast cost {2} less to cast" and "{T}: Draw a card."
//!
//! GAP: Compleated ({G/U/P} alternative-payment / "enters with two fewer
//!   loyalty" rider) is not expressible from the demonstrated cost surface;
//!   the printed starting loyalty 3 is recorded as-is.
//! GAP: the +1 "doesn't untap during its controller's next untap step" rider is
//!   not expressible; the tap itself is emitted as best-effort.
//! GAP: the −X ability is OMITTED — a chosen-X loyalty cost is not expressible
//!   (`remove_self_counter` is a fixed u32).
//! GAP: the −7 token-creation effect cannot construct the bespoke
//!   "Tamiyo's Notebook" Book artifact token from the demonstrated APIs; the
//!   ability is declared with its correct cost but its effect is empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tamiyo, Compleated Sage");
    let tamiyo = reg.interner_mut().intern("Tamiyo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tamiyo);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G/U/P}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // +1: Tap up to one target artifact or creature. (No-untap rider GAP'd.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Tap up to one target artifact or creature. It doesn't untap during its controller's next untap step.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
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
            // −X: OMITTED — dynamic-X loyalty cost is not expressible.
            // GAP: dynamic-X loyalty cost
            // −7: Create Tamiyo's Notebook token. (Bespoke token GAP'd.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Create Tamiyo's Notebook, a legendary colorless Book artifact token with \"Spells you cast cost {2} less to cast\" and \"{T}: Draw a card.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_notebook,
            }),
    )
}

/// `+1: Tap up to one target artifact or creature.`
fn plus_one_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: the "doesn't untap during its controller's next untap step" rider
    // is not expressible; emitting the tap as best-effort.
    vec![Effect::Tap { target: *id }]
}

/// `-7: Create Tamiyo's Notebook ...`
fn minus_seven_notebook(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot construct the bespoke "Tamiyo's Notebook" Book artifact token
    // (custom abilities, cost reduction static) from the demonstrated APIs.
    Vec::new()
}
