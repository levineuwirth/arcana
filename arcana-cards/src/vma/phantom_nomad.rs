//! Phantom Nomad — `{1}{W}` 0/0 white Spirit Nomad.
//! Enters with two +1/+1 counters. "If damage would be dealt to this
//! creature, prevent that damage. Remove a +1/+1 counter from it."
//!
//! The enters-with-counters clause is a SelfEntersBattlefield trigger that
//! adds the counters to itself. The damage-prevention replacement static is
//! GAP'd — there is no static "if damage would be dealt to ~, prevent it and
//! remove a counter" continuous/replacement primitive in the allowed API
//! (Effect::PreventDamage is a one-shot, not a printed static ability).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phantom Nomad");
    let spirit = reg.interner_mut().intern("Spirit");
    let nomad = reg.interner_mut().intern("Nomad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(nomad);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    // GAP: "If damage would be dealt to this creature, prevent that damage.
    // Remove a +1/+1 counter from this creature." — a printed damage-
    // replacement static is not expressible with the allowed primitives.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_two_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_two_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
