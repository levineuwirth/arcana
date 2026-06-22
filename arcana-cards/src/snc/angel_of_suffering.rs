//! Angel of Suffering — `{3}{B}{B}` 5/3 Nightmare Angel with Flying.
//!
//! * Flying — keyword.
//! * "If damage would be dealt to you, prevent that damage and mill twice
//!   that many cards." — GAP: a continuous damage-replacement on its
//!   controller (prevent + mill 2×) is a static replacement effect with no
//!   trigger/activated shape and no expressible primitive on this card
//!   class. (Scryfall's "Mill" keyword is not a `KeywordAbility` variant.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angel of Suffering");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
