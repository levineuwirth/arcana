//! Sundering Fork — artifact — Contraption (no mana cost).
//! "Whenever you crank this Contraption, destroy target artifact."
//!
//! // GAP: trigger — "crank this Contraption" (Un-set Contraption
//! // mechanic) is not modeled; the closest available condition
//! // (`SelfBecomesTapped`) is used as a placeholder.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sundering Fork");
    let contraption = reg.interner_mut().intern("Contraption");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — no "crank" event exists (Un-set
                // Contraption mechanic); SelfBecomesTapped is the closest
                // placeholder.
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: destroy_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…destroy target artifact."
fn destroy_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}
