//! Curious Obsession — `{U}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/+1 and has \"Whenever
//! this creature deals combat damage to a player, you may draw a card.\"
//! At the beginning of your end step, if you didn't attack with a
//! creature this turn, sacrifice this Aura."
//!
//! The +1/+1 buff is the standard `attached_pt`. The GRANTED triggered
//! ability ("combat damage to a player → draw") still needs source-bound
//! grant support. The end-step sacrifice checks this turn’s attack events.

use arcana_core::effects::Effect;
use arcana_core::events::GameEvent;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
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
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: the combat-damage draw still needs a source-bound trigger grant.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
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
