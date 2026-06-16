//! Allosaurus Rider — `{5}{G}{G}` Elf Warrior, power/toughness each equal
//! to 1 plus the number of lands you control.
//!
//! Both pieces of rules text are unexpressible with the demonstrated API:
//! the alternative cost ("exile two green cards rather than pay this spell's
//! mana cost") and the characteristic-defining power/toughness ("each equal
//! to 1 plus the number of lands you control") have no primitive in the
//! MultiAbilityCreature surface. Only the bones are emitted; printed P/T set
//! to the base 1/1 (the "1 plus" floor).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: alternative cost "You may exile two green cards from your hand rather
// than pay this spell's mana cost" — no alt-cost primitive available.
// GAP: characteristic-defining P/T "each equal to 1 plus the number of lands
// you control" — no CDA P/T primitive in this surface; printed as Fixed(1).

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Allosaurus Rider");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
