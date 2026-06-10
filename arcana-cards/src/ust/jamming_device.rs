//! Jamming Device — Artifact — Contraption (no mana cost).
//! "Whenever you crank this Contraption, creatures target player
//! controls get -1/-1 until end of turn." The crank mechanic (Unstable
//! Contraptions) is not modeled — see GAP on the trigger condition.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jamming Device");
    let contraption = reg.interner_mut().intern("Contraption");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "Whenever you crank this Contraption"
                // (Unstable crank mechanic not modeled);
                // SelfBecomesTapped is the closest available condition.
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: shrink_their_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            },
        ),
    )
}

fn shrink_their_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        *p,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: -1,
            toughness: -1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
