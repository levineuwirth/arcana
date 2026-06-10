//! Dwarven Soldier — `{1}{R}` 2/1 Dwarf Soldier.
//! "Whenever this creature blocks or becomes blocked by one or more
//! Orcs, this creature gets +0/+2 until end of turn."
//!
//! Wired with `SelfBlocksOrBecomesBlockedBy` and an Orc-subtype filter:
//! fires only when the paired creature(s) include at least one Orc.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwarven Soldier");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let soldier = reg.interner_mut().intern("Soldier");
    let orc = reg.interner_mut().intern("Orc");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "blocks or becomes blocked by one or more Orcs" — filtered
                // either-direction form; the paired creature(s) must include an Orc.
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlockedBy {
                    filter: ObjectFilter::creature().with_subtype_sym(orc),
                },
                intervening_if: None,
                effect: on_blocks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_blocks(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 0,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
