//! Deepmuck Desperado — `{2}{U}` 2/4 blue Homarid Mercenary. "Whenever you
//! commit a crime, each opponent mills three cards. This ability triggers
//! only once each turn."
//!
//! GAP: no TriggerCondition for "commit a crime". Best-effort trigger with
//! ZoneChange as closest proxy not available — using SpellCast opponent as
//! closest; GAP noted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deepmuck Desperado");
    let homarid = reg.interner_mut().intern("Homarid");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homarid);
    subtypes.0.insert(mercenary);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — no TriggerCondition::CrimeCommitted variant
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: arcana_core::targets::ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_crime,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_crime(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 3 })
        .collect()
}
