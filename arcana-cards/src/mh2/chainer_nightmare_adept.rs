//! Chainer, Nightmare Adept — `{2}{B}{R}` 3/2 Legendary Human Minion.
//! Discard a card: You may cast a creature spell from your graveyard this turn.
//!   Activate only once each turn.
//! Whenever a nontoken creature you control enters, if you didn't cast it from your
//!   hand, it gains haste until your next turn.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chainer, Nightmare Adept");
    let human = reg.interner_mut().intern("Human");
    let minion = reg.interner_mut().intern("Minion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(minion);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a card: You may cast a creature spell from your graveyard this turn. Activate only once each turn."
                    .into(),
                cost: ActivationCost {
                    discard_other: Some(ObjectFilter::default()),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                // GAP: "you may cast a creature spell from your graveyard this turn" grants a
                // cast-from-graveyard permission — no Effect grants that permission window.
                effect: grant_cast_from_gy,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .nontoken()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                // GAP: intervening-if "if you didn't cast it from your hand" — no condition
                // predicate for the cast-source of the entering creature.
                intervening_if: None,
                effect: grant_haste,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn grant_cast_from_gy(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}

fn grant_haste(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: id,
        keyword: arcana_core::effects::KeywordAbility::Haste,
        duration: arcana_core::layers::Duration::UntilYourNextTurn(trig.controller),
    }]
}
