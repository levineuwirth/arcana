//! Sigarda, Font of Blessings — `{2}{G}{W}` 4/4 Legendary Angel (G/W). Flying.
//! "Other permanents you control have hexproof."
//! "You may look at the top card of your library any time."
//! "You may cast Angel spells and Human spells from the top of your library."
//!
//! Flying is a base keyword. All three remaining lines are static continuous
//! abilities (a hexproof anthem, a top-of-library reveal permission, and a
//! play-from-top permission); none decompose into a triggered or activated
//! ability or an Effect, so all three are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sigarda, Font of Blessings");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Other permanents you control have hexproof" — static hexproof anthem.
    // GAP: "You may look at the top card of your library any time" — static.
    // GAP: "You may cast Angel spells and Human spells from the top of your
    // library" — static play-from-top permission.
    reg.register(CardDefinition::new(name, chars))
}
