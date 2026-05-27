//! Freelance Muscle — `{4}{G}` 4/4 green Rhino Warrior.
//! "Whenever this creature attacks or blocks, it gets +X/+X until end of turn,
//! where X is the greatest power and/or toughness among other creatures you control."
//! GAP: "greatest power and/or toughness" — script helpers only provide individual power/toughness
//! per creature, not max over all controlled creatures. Cannot compute without iteration over ids.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Freelance Muscle");
    let rhino = reg.interner_mut().intern("Rhino");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "attacks or blocks" — SelfAttacks covers attack; no combined attacks-or-blocks trigger.
                // Use SelfAttacks as partial approximation.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_by_max_pt,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_by_max_pt(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    let mut max_val: i32 = 0;
    for id in ids {
        if id == trig.source { continue; }
        let p = script::power_of(state, id);
        let t = script::toughness_of(state, id);
        let val = p.max(t);
        if val > max_val { max_val = val; }
    }
    let x = max_val.max(0) as i32;
    vec![Effect::Pump {
        target: trig.source,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
