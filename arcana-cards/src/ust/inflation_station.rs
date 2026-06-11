//! Inflation Station — artifact — Contraption (Unstable, 2017).
//! "Whenever you crank this Contraption, target creature gets +3/+3
//! until end of turn."
//!
//! Contraptions have no mana cost (they are assembled, not cast) —
//! `mana_cost: None`. The crank mechanic is not modeled by any
//! `TriggerCondition`; the trigger is wired on the closest (inert for
//! a non-creature artifact) `SelfBecomesTapped` placeholder with an
//! honest GAP, and the pump payoff is wired faithfully.

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
    let name = reg.interner_mut().intern("Inflation Station");
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
                // GAP: trigger — "Whenever you crank this Contraption" (the
                // Unstable crank/assemble mechanic) has no TriggerCondition;
                // SelfBecomesTapped is the closest placeholder and never
                // fires for a non-creature artifact that is never tapped.
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

/// "…target creature gets +3/+3 until end of turn."
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
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
