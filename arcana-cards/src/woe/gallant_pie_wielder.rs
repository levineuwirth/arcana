//! Gallant Pie-Wielder — `{2}{W}` 2/3 white Dwarf Knight.
//! First strike.
//! Celebration — This creature has double strike as long as two or more
//! nonland permanents entered the battlefield under your control this turn.
//!
//! First strike is a base keyword. Celebration is an ability-word, not a
//! `KeywordAbility` variant; the conditional double-strike grant is a
//! continuous static (no trigger word, no cost) and is not expressible via
//! the MultiAbilityCreature surface.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gallant Pie-Wielder");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };
    // GAP: "Celebration — has double strike as long as two or more nonland
    // permanents entered under your control this turn" is a conditional
    // continuous static keyword grant, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
