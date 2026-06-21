//! Viscera Dragger — `{3}{B}` 3/3 Zombie Ogre Warrior.
//! Cycling {2} (synthesized from the keyword). Unearth {1}{B} is GAP'd —
//! it is not part of the usable `KeywordAbility` surface.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Viscera Dragger");
    let zombie = reg.interner_mut().intern("Zombie");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    // GAP: Unearth {1}{B} — no KeywordAbility::Unearth; the graveyard
    // return-with-haste-then-exile activation is not expressible.

    reg.register(CardDefinition::new(name, chars))
}
