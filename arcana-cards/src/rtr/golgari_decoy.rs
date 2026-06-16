//! Golgari Decoy — `{3}{G}` 2/2 Elf Rogue.
//! All creatures able to block this creature do so. (Lure-style static — GAP)
//! Scavenge {3}{G}{G} (the engine synthesizes the graveyard activated ability
//! from this keyword).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Golgari Decoy");
    let elf = reg.interner_mut().intern("Elf");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Scavenge(
            ManaCost::parse("{3}{G}{G}").expect("valid cost"),
        )],
        // GAP: static "All creatures able to block this creature do so" (Lure).
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
