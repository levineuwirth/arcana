//! Changeling Titan — `{4}{G}` 7/7 green Shapeshifter with Changeling.
//! "Champion a creature (When this enters, sacrifice it unless you exile another
//!  creature you control. When this leaves the battlefield, that card returns.)"
//!
//! Changeling is a base keyword. The Champion mechanic (a paired enters/leaves
//! exile-and-return) is not in the usable keyword surface and has no Effect
//! representation, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Changeling Titan");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    // GAP: "Champion a creature" — the enters-sacrifice-unless-exile / leaves-return
    // paired mechanic is not in the usable keyword surface and has no Effect form.
    reg.register(CardDefinition::new(name, chars))
}
