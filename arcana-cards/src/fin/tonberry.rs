//! Tonberry — `{B}` 2/1 Salamander Horror.
//! "This creature enters tapped with a stun counter on it.
//!  Chef's Knife — During your turn, this creature has first strike and
//!  deathtouch."
//!
//! "Enters tapped with a stun counter" is modeled as an ETB trigger that
//! taps this creature and adds a stun counter (close to the printed
//! enters-with modifier). The conditional static "during your turn, has
//! first strike and deathtouch" has no conditional keyword-grant effect
//! in this card class — GAP'd. "Chef's Knife" is an ability-word label,
//! not a usable keyword.

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
    let name = reg.interner_mut().intern("Tonberry");
    let salamander = reg.interner_mut().intern("Salamander");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(salamander);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: enters_tapped_with_stun,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
    // GAP: "Chef's Knife — During your turn, this creature has first
    // strike and deathtouch" — a turn-conditional keyword static with no
    // conditional keyword-grant effect in this card class.
}

fn enters_tapped_with_stun(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Tap { target: trig.source },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Stun,
            count: 1,
        },
    ]
}
