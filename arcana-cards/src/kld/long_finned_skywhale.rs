//! Long-Finned Skywhale — `{2}{U}{U}` 4/3 Whale with Flying.
//! "This creature can block only creatures with flying."
//!
//! Flying is a base characteristic. The block-restriction static
//! ("can block only creatures with flying") is GAP'd — there is no
//! effect/primitive to constrain which creatures this one may block.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Long-Finned Skywhale");
    let whale = reg.interner_mut().intern("Whale");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(whale);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "This creature can block only creatures with flying." —
    // a static blocking restriction with no expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}
