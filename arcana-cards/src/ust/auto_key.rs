//! Auto-Key — artifact — Contraption (Unstable, 2017).
//! "Whenever you crank this Contraption, until end of turn, target
//! creature becomes an artifact in addition to its other types and
//! gains "{T}: You gain 3 life.""
//!
//! GAP: the crank/assemble Contraption mechanic is not modeled — the
//! trigger uses the closest available condition (SelfBecomesTapped).
//! The granted activated ability is also inexpressible (only
//! triggered abilities can be granted).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Auto-Key");
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
                // GAP: trigger — 'whenever you crank this Contraption'
                // (crank is not modeled; SelfBecomesTapped is the
                // closest available condition).
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: crank_animate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            },
        ),
    )
}

fn crank_animate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: 'gains "{T}: You gain 3 life."' — no grant-activated-ability
    // primitive (GrantTriggeredAbility covers triggered abilities only).
    vec![Effect::AddType {
        target: *id,
        types: TypeLine::ARTIFACT.into(),
        duration: Duration::EndOfTurn,
    }]
}
