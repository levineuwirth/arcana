//! Klauth, Unrivaled Ancient — `{5}{R}{G}` 4/4 Legendary Dragon.
//! Flying, haste.
//! "Whenever Klauth attacks, add X mana in any combination of colors,
//! where X is the total power of attacking creatures. Spend this mana
//! only to cast spells. Until end of turn, you don't lose this mana as
//! steps and phases end." — the attack trigger is wired but its effect
//! is GAP'd: no script helper for total attacker power, AddMana takes a
//! fixed color vec (not "any combination"), and the spend/persist
//! restrictions are not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Klauth, Unrivaled Ancient");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: add_attacker_power_mana,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn add_attacker_power_mana(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "add X mana in any combination of colors, where X = total power
    // of attacking creatures, spend only to cast spells, persists across
    // steps/phases" — no script helper for total attacker power, AddMana
    // requires a fixed color vec, and the spend/persist riders are not
    // expressible with the demonstrated API.
    Vec::new()
}
