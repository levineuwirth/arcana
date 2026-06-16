//! Tamiyo, Field Researcher — `{1}{G}{W}{U}` Legendary Planeswalker — Tamiyo,
//! starting loyalty 4.
//!
//! Loyalty abilities:
//! * `+1`: Choose up to two target creatures; until your next turn, whenever
//!   either deals combat damage, you draw a card. GAP — the floating
//!   "until your next turn, on combat damage draw" rider on chosen creatures is
//!   not expressible. Ability shell declared.
//! * `−2`: Tap up to two target nonland permanents. They don't untap during
//!   their controller's next untap step. PARTIAL — the Tap is wired; the
//!   "don't untap next untap step" rider is not expressible (documented).
//! * `−7`: Draw three cards. You get an emblem with "You may cast spells from
//!   your hand without paying their mana costs." PARTIAL — Draw 3 wired; the
//!   emblem is a GAP (cast-without-paying static not expressible).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tamiyo, Field Researcher");
    let tamiyo = reg.interner_mut().intern("Tamiyo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tamiyo);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Choose up to two target creatures. Until your next \
                       turn, whenever either of those creatures deals combat \
                       damage, you draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Tap up to two target nonland permanents. They don't \
                       untap during their controller's next untap step.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_tap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Draw three cards. You get an emblem with \"You may \
                       cast spells from your hand without paying their mana \
                       costs.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ultimate_draw,
            }),
    )
}

/// `+1`: combat-damage-draw rider on up to two creatures.
fn plus_one(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "until your next turn, whenever either chosen creature deals combat
    // damage, draw a card" — floating per-target rider not expressible here.
    Vec::new()
}

/// `−2: Tap up to two target nonland permanents.` (Don't-untap rider GAP'd.)
fn minus_two_tap(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // PARTIAL: the "don't untap during their controller's next untap step"
    // rider is not expressible; only the Tap is wired.
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::Tap { target: *id }),
            _ => None,
        })
        .collect()
}

/// `−7: Draw three cards.` (Emblem GAP'd.)
fn ultimate_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem "you may cast spells from your hand without paying their mana
    // costs" — cast-without-paying static not expressible. Draw 3 wired.
    vec![Effect::DrawCards { player: ctx.controller, count: 3 }]
}
