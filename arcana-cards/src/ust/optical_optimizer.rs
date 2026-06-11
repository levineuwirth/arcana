//! Optical Optimizer — artifact — Contraption (no mana cost).
//! "Whenever you crank this Contraption, until end of turn, target
//! creature becomes an artifact in addition to its other types and
//! gains '{T}: Draw a card.'"
//!
//! GAP: trigger — "Whenever you crank this Contraption" (the
//! crank/sprocket Contraption mechanic is unmodeled); wired on the
//! closest available condition, `SelfEntersBattlefield`.

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
    let name = reg.interner_mut().intern("Optical Optimizer");
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
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "Whenever you crank this Contraption" has no
            // TriggerCondition variant; SelfEntersBattlefield is the
            // closest available stand-in.
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: animate_to_artifact,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

/// "…until end of turn, target creature becomes an artifact in addition
/// to its other types and gains '{T}: Draw a card.'"
fn animate_to_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: granting an ACTIVATED ability ("{T}: Draw a card.") is not
    // expressible (GrantTriggeredAbility covers triggered abilities
    // only); only the type addition is emitted.
    vec![Effect::AddType {
        target: *id,
        types: TypeLine::ARTIFACT.into(),
        duration: Duration::EndOfTurn,
    }]
}
