//! Garruk, Savage Herald — `{4}{G}{G}` legendary planeswalker, starting loyalty 4.
//! Subtype Garruk; mono-green.
//!
//! Loyalty abilities:
//! * `+1`: Reveal the top card of your library. If it's a creature card,
//!   put it into your hand. Otherwise, put it on the bottom of your
//!   library. (Modeled with `Effect::DigTopN` over a creature filter,
//!   rest to the bottom.)
//! * `−2`: Target creature you control deals damage equal to its power to
//!   another target creature. GAP — one-directional power-based damage
//!   between two distinct targets isn't expressible (Fight is mutual; no
//!   power-as-amount one-way primitive).
//! * `−7`: Until end of turn, creatures you control gain "assign combat
//!   damage as though unblocked." GAP — no such grantable keyword/effect.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk, Savage Herald");
    let garruk = reg.interner_mut().intern("Garruk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(garruk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Reveal the top card of your library. If it's a \
                       creature card, put it into your hand. Otherwise, put \
                       it on the bottom of your library."
                    .into(),
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
                effect: plus_one_reveal,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Target creature you control deals damage equal to \
                       its power to another target creature."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Until end of turn, creatures you control gain \"You \
                       may have this creature assign its combat damage as \
                       though it weren't blocked.\""
                    .into(),
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
                effect: minus_seven_gap,
            }),
    )
}

fn plus_one_reveal(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 1,
        filter: Some(ObjectFilter::creature()),
        rest: DigRest::BottomRandom,
    }]
}

fn minus_two_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: one-directional power-based damage between two distinct targets
    // is not expressible (Fight is mutual; no power-as-amount one-way effect).
    Vec::new()
}

fn minus_seven_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: granting "assign combat damage as though unblocked" — no such
    // keyword/continuous effect in the demonstrated surface.
    Vec::new()
}
