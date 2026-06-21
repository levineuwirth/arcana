//! Qarsi Sadist — `{1}{B}` 1/3 Creature — Human Cleric.
//! Exploit (When this creature enters, you may sacrifice a creature.)
//! When this creature exploits a creature, target opponent loses 2 life
//!   and you gain 2 life.
//!
//! GAP: Exploit is not modeled — the "when this enters, you may
//! sacrifice a creature" cost and the resulting "when this exploits a
//! creature" event are absent, so the lose-2/gain-2 payoff cannot be
//! gated on an exploit and is not emitted (firing it unconditionally on
//! ETB would be a materially wrong card).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Qarsi Sadist");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Exploit is not an expressible KeywordAbility.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: exploit_payoff,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn exploit_payoff(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Exploit mechanic not modeled — cannot sacrifice-then-trigger, so
    // the "target opponent loses 2 life, you gain 2 life" payoff is omitted.
    Vec::new()
}
