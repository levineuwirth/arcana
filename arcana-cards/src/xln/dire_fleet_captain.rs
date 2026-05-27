//! Dire Fleet Captain — `{B}{R}` 2/2 black/red Creature — Orc Pirate.
//! "Whenever this creature attacks, it gets +1/+1 until end of turn for each other attacking Pirate."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dire Fleet Captain");
    let orc = reg.interner_mut().intern("Orc");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(pirate);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_pump_per_pirate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_pump_per_pirate(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Count other attacking Pirates you control — use all Pirates you control minus 1 (self)
    // GAP: no "attacking" filter on ObjectFilter; using all Pirates you control as approximation.
    let pirate_filter = script::subtype_filter(reg, "Pirate")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &pirate_filter, trig.controller).saturating_sub(1);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::Pump {
        target: trig.source,
        power: n as i32,
        toughness: n as i32,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
