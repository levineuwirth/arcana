//! Drainpipe Vermin — `{B}` 1/1 Rat. "When this creature dies, you may pay
//! {B}. If you do, target player discards a card."
//!
//! GAP: optional mana payment as a cost gating the discard Effect is not
//! expressible in the Effect catalog; emitting discard unconditionally.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drainpipe Vermin");
    let rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn on_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional {B} payment gating discard is not expressible.
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(player) = target else { return Vec::new(); };
    vec![Effect::Discard {
        player: *player,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
