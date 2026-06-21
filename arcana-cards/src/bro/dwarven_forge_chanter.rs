//! Dwarven Forge-Chanter — `{1}{R}` 1/3 red Dwarf Wizard.
//! Ward—Pay 2 life. Prowess.
//!
//! Both printed keywords are unexpressible with the documented surface:
//! - Ward with a NON-mana cost ("Ward—Pay 2 life") has no KeywordAbility
//!   form (only `Ward(ManaCost)` exists). GAP.
//! - Prowess is not in the usable keyword set. GAP.
//! Bones recorded; no abilities emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwarven Forge-Chanter");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(wizard);

    // GAP: Ward—Pay 2 life (non-mana ward cost not expressible as KeywordAbility::Ward).
    // GAP: Prowess (not in the usable keyword set).
    let keywords: Vec<KeywordAbility> = vec![];

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords,
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
