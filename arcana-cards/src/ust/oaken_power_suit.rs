//! Oaken Power Suit — artifact, subtype Contraption.
//! "Whenever you crank this Contraption, target creature gets +X/+X
//! until end of turn, where X is the greatest power among creatures
//! you control."
//!
//! GAP: trigger — "Whenever you crank this Contraption" (the
//! Contraption crank mechanic is not modeled; no crank
//! TriggerCondition exists). The closest available self-event
//! condition `SelfBecomesTapped` is used as a placeholder; the
//! dynamic pump is computed faithfully.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Oaken Power Suit");
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
    reg.register(CardDefinition::new(name, chars).with_triggered_ability(
        TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "Whenever you crank this Contraption" has no
            // TriggerCondition (crank is unmodeled); SelfBecomesTapped is
            // the closest placeholder.
            trigger_condition: TriggerCondition::SelfBecomesTapped,
            intervening_if: None,
            effect: crank_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        },
    ))
}

fn crank_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let x = ids
        .into_iter()
        .map(|cid| script::power_of(state, cid))
        .max()
        .unwrap_or(0)
        .max(0);
    vec![Effect::Pump {
        target: *id,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
