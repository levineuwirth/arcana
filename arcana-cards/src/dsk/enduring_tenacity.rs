//! Enduring Tenacity — `{2}{B}{B}` 4/3 black Enchantment Creature — Snake Glimmer.
//!
//! Whenever you gain life, target opponent loses that much life.
//!   (GAP: the dynamic amount "that much" — the amount of life gained — is not
//!   exposed by a PendingTrigger accessor, so the loss cannot be computed;
//!   targeted trigger recorded with empty effect.)
//! When Enduring Tenacity dies, if it was a creature, return it to the battlefield
//!   under its owner's control. It's an enchantment (not a creature).
//!   (GAP: the "becomes an enchantment, not a creature" type-change rider on the
//!   returned permanent is not expressible; the graveyard return is emitted.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enduring Tenacity");
    let snake = reg.interner_mut().intern("Snake");
    let glimmer = reg.interner_mut().intern("Glimmer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(glimmer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_life_gained,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_life_gained(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses that much life" — the gained amount is not exposed by a
    // PendingTrigger accessor, so the dynamic loss cannot be computed.
    Vec::new()
}

fn on_dies_return(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "It's an enchantment (not a creature)" type-change rider not expressible.
    let id = trig.dying_object().unwrap_or(trig.source);
    vec![Effect::ReturnFromGraveyardToBattlefield { target: id }]
}
