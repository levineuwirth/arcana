//! Buzz Buggy — artifact — Contraption (Unstable, 2017).
//! "Whenever you crank this Contraption, target creature gets +2/+0
//! and gains trample until end of turn."
//!
//! GAP: the crank/assemble Contraption mechanic is not modeled — the
//! trigger uses the closest available condition (SelfBecomesTapped).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Buzz Buggy");
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
                effect: crank_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            },
        ),
    )
}

fn crank_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample],
    }]
}
