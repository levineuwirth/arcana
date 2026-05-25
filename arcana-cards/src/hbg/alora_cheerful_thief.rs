//! Alora, Cheerful Thief — `{3}{U}{U}` 4/4 legendary blue Halfling Rogue.
//! "Whenever you attack, up to one target attacking creature can't be blocked this turn.
//! At the beginning of the next end step, return that creature to its owner's hand.
//! If you do, a creature of your choice an opponent controls perpetually gets -1/-0."
//! GAP: "can't be blocked" effect, perpetual -1/-0 debuff, and multi-step conditional
//! not in engine effect catalog. Best-effort: bounce the targeting creature at next end step.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alora, Cheerful Thief");
    let halfling = reg.interner_mut().intern("Halfling");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: arcana_core::targets::ObjectFilter::creature()
                        .controlled_by(arcana_core::targets::ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn on_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "can't be blocked" and perpetual -1/-0 not in engine.
    // Best-effort: schedule a bounce at end step.
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        vec![Effect::DelayedAction {
            source: trig.source,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnToHand,
        }]
    } else {
        Vec::new()
    }
}
