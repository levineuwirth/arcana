//! Ascending Aven — `{2}{U}{U}` 3/2 Bird Soldier.
//! Flying. The static "This creature can block only creatures with flying" is a
//! filtered block restriction with no expressible primitive — GAP'd. Morph {2}{U}
//! is not in the usable keyword surface and the face-down cast is not modeled — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ascending Aven");
    let bird = reg.interner_mut().intern("Bird");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Morph {2}{U} (not in usable keyword surface; face-down cast not modeled).
        ..Default::default()
    };

    // GAP: static "This creature can block only creatures with flying" (filtered
    // block restriction).
    reg.register(CardDefinition::new(name, chars))
}
