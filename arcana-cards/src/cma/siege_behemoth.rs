//! Siege Behemoth — `{5}{G}{G}` 7/4 Beast with Hexproof.
//! "As long as this creature is attacking, for each creature you control, you
//! may have that creature assign its combat damage as though it weren't
//! blocked."
//!
//! Hexproof is a base keyword. The conditional combat-damage-assignment static
//! ("assign damage as though it weren't blocked") has no expressible primitive
//! in this class — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Siege Behemoth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    // GAP: static "while attacking, your creatures may assign combat damage as
    // though not blocked" — no combat-damage-assignment-override primitive.
    reg.register(CardDefinition::new(name, chars))
}
