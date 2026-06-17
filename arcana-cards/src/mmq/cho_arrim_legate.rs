//! Cho-Arrim Legate — `{2}{W}` 1/2 Human Soldier.
//! "If an opponent controls a Swamp and you control a Plains, you may cast this
//! spell without paying its mana cost." (alternative-cost permission — GAP)
//! "Protection from black" (keyword not in the usable surface — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cho-Arrim Legate");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Protection from black is not in the usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: alternative-cost permission — "you may cast this spell without paying
    // its mana cost if ..." is a static casting permission, not a
    // triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
