//! Flourishing Defenses — `{4}{G}` enchantment.
//! "Whenever a -1/-1 counter is put on a creature, you may create a 1/1
//! green Elf Warrior creature token."
//!
//! GAP: trigger — `CounterAdded` only offers `TriggerSelf::Source` in
//! the catalog, so "a -1/-1 counter is put on A CREATURE (any)" is
//! narrowed to counters on this enchantment itself (conservative —
//! effectively never fires). GAP: "you may" — the optional choice has
//! no costless gate; the token creation is emitted as mandatory.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf,
    TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flourishing Defenses");
    let _elf = reg.interner_mut().intern("Elf");
    let _warrior = reg.interner_mut().intern("Warrior");
    let _token_name = reg.interner_mut().intern("Elf Warrior");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CounterAdded {
                on: TriggerSelf::Source,
                kind: Some(CounterKind::MinusOneMinusOne),
                chapter: None,
            },
            intervening_if: None,
            effect: sprout_elf_warrior,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…you may create a 1/1 green Elf Warrior creature token."
fn sprout_elf_warrior(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(token_name) = reg.interner().lookup("Elf Warrior") else {
        return Vec::new();
    };
    let Some(elf) = reg.interner().lookup("Elf") else {
        return Vec::new();
    };
    let Some(warrior) = reg.interner().lookup("Warrior") else {
        return Vec::new();
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
