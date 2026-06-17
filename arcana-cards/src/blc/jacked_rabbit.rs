//! Jacked Rabbit — `{X}{1}{W}` 1/2 Rabbit Warrior.
//! Ravenous (enters with X +1/+1 counters; if X >= 5 draw a card on ETB).
//! "Whenever this creature attacks, create a number of 1/1 white Rabbit
//!  creature tokens equal to this creature's power."
//!
//! Ravenous (enters-with-X-counters + conditional draw) is not in the
//! usable keyword surface — see GAP. The attack trigger creates one Rabbit
//! token per point of this creature's current power.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jacked Rabbit");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(warrior);

    // GAP: Ravenous — "enters with X +1/+1 counters; if X is 5 or more,
    // draw a card when it enters" is not expressible (no enters-with-X
    // counter primitive nor an X-spent accessor).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: make_rabbits,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_rabbits(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::power_of(state, trig.source).max(0) as u32;
    let rabbit = reg.interner().lookup("Rabbit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    let token = TokenDefinition {
        name: rabbit,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}
