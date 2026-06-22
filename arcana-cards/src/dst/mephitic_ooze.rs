//! Mephitic Ooze — `{4}{B}` 0/5 black Ooze.
//! This creature gets +1/+0 for each artifact you control.
//! Whenever this creature deals combat damage to a creature, destroy that
//! creature. The creature can't be regenerated.
//!
//! The self-buffing static "+1/+0 for each artifact you control" is a dynamic-
//! P/T continuous static with no triggered/activated form — GAP'd. The
//! combat-damage trigger condition is wired, but its payload "destroy that
//! creature" is GAP'd: no PendingTrigger accessor recovers the creature this
//! dealt combat damage to (only damaged_player() exists).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mephitic Ooze");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static "This creature gets +1/+0 for each artifact you control" —
    //      a dynamic-P/T continuous static with no triggered/activated form.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Creature,
                combat_only: true,
            },
            intervening_if: None,
            effect: destroy_damaged_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn destroy_damaged_creature(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "destroy that creature; can't be regenerated" — no PendingTrigger
    //      accessor recovers the creature this dealt combat damage to (only
    //      damaged_player()), so the damaged-creature id can't be recovered.
    Vec::new()
}
