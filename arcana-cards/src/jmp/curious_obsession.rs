//! Curious Obsession — `{U}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/+1 and has \"Whenever
//! this creature deals combat damage to a player, you may draw a card.\"
//! At the beginning of your end step, if you didn't attack with a
//! creature this turn, sacrifice this Aura."
//!
//! "Enchanted creature gets +1/+1 and has …" is one printed static of two
//! parts, the `attached_pt` buff and the granted triggered ability, which
//! the engine installs as the Aura enters (A1.5br). The ability belongs to
//! the enchanted creature: its controller draws, and it ends when the Aura
//! leaves or is no longer attached. The end-step sacrifice checks this
//! turn’s attack events.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::events::{DamageTarget, GameEvent};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Curious Obsession");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_static(vec![
                ContinuousEffect::attached_pt(
                    NULL_OBJECT_ID,
                    1,
                    1,
                    Duration::WhileSourceOnBattlefield,
                ),
                ContinuousEffect::attached_grant_triggered(
                    NULL_OBJECT_ID,
                    TriggeredAbilityDef {
                        id: GRANTED_TRIGGER_ID_BASE + 1,
                        trigger_condition: TriggerCondition::Custom(
                            host_deals_combat_damage_to_player),
                        intervening_if: None,
                        effect: may_draw,
                        trigger_zones: vec![Zone::Battlefield],
                        frequency: TriggerFrequency::EachTime,
                        target_requirements: Vec::new(),
                    },
                    Duration::WhileSourceOnBattlefield,
                ),
            ])
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(did_not_attack),
                effect: sacrifice_obsession,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

/// "Whenever this creature deals combat damage to a player": `source` is
/// the enchanted creature, which has the ability.
fn host_deals_combat_damage_to_player(
    event: &GameEvent,
    _state: &GameState,
    source: ObjectId,
) -> bool {
    matches!(event, GameEvent::DamageDealt {
        source: dealer, target: DamageTarget::Player(_), is_combat: true, ..
    } if *dealer == source)
}

/// "You may draw a card": the enchanted creature's controller chooses.
fn may_draw(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Life(0),
        then: Box::new(Effect::DrawCards { player: trig.controller, count: 1 }),
        else_effect: None,
    }]
}

fn did_not_attack(
    state: &GameState,
    _source: ObjectId,
    controller: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // This check is made at your own end step. Only the active player
    // declares attackers, so their current controller or LKI is irrelevant.
    state.active_player() == controller
        && !state
            .event_log
            .iter()
            .skip(state.turn_event_log_start)
            .any(|event| matches!(event, GameEvent::CreatureAttacks { .. }))
}

fn sacrifice_obsession(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    if state
        .objects
        .get(trig.source)
        .is_some_and(|obj| obj.controller == trig.controller)
    {
        vec![Effect::SacrificePermanent {
            target: trig.source,
        }]
    } else {
        vec![]
    }
}
