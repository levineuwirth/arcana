//! Unbreathing Horde — `{2}{B}` 0/0 Zombie.
//!
//! Oracle:
//! * This creature enters with a +1/+1 counter on it for each other
//!   Zombie you control and each Zombie card in your graveyard.
//! * If this creature would be dealt damage, prevent that damage and
//!   remove a +1/+1 counter from it.  (replacement static — GAP)
//!
//! The dynamic ETB counters are wired: an ETB trigger places counters
//! equal to (Zombies you control, minus this one) + (Zombie cards in
//! your graveyard).
//!
//! GAP: "If this creature would be dealt damage, prevent that damage and
//! remove a +1/+1 counter from it" is a self-installed damage-prevention
//! replacement effect; no triggered/activated primitive expresses a
//! permanent self-replacement of this shape.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unbreathing Horde");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_counters(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let zombie_filter = script::subtype_filter(reg, "Zombie");
    // Zombies you control (count_matching includes this creature, which
    // has just entered) minus this one → "each OTHER Zombie you control".
    let on_field = script::count_matching(
        state,
        &zombie_filter.clone().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let other_on_field = on_field.saturating_sub(1);
    // Zombie cards in your graveyard.
    let in_graveyard =
        script::graveyard_matching(state, &zombie_filter, trig.controller, trig.controller);
    let total = other_on_field + in_graveyard;
    if total == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: total,
    }]
}
