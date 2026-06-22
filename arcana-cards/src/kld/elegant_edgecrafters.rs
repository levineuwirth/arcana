//! Elegant Edgecrafters — `{4}{G}{G}` 3/4 green Elf Artificer.
//!
//! This creature can't be blocked by creatures with power 2 or less.
//! (GAP — power-filtered evasion static is not expressible.)
//! Fabricate 2 (When this creature enters, put two +1/+1 counters on it
//! or create two 1/1 Servo tokens). Modeled as an ETB trigger that
//! places the two +1/+1 counters; the token alternative is GAP'd.

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
    let name = reg.interner_mut().intern("Elegant Edgecrafters");
    let elf = reg.interner_mut().intern("Elf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Fabricate is not in the usable keyword surface for this class.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "can't be blocked by creatures with power 2 or less" — a
    // power-filtered evasion static has no engine representation here.

    reg.register(
        CardDefinition::new(name, chars)
            // Fabricate 2: ETB choice. Model the put-two-+1/+1-counters
            // branch; the create-two-Servo-tokens alternative is GAP'd
            // (no modal-ETB-choice machinery for triggers).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: fabricate_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn fabricate_two(
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
