//! Skyshroud Behemoth — `{5}{G}{G}` 10/10 Creature — Beast.
//! "Fading 2." "This creature enters tapped."
//!
//! Fading 2 is a parametrized keyword (`KeywordAbility::Fading(2)`). The
//! "enters tapped" line is a static entering replacement with no expressible
//! primitive in this card class, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyshroud Behemoth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    // GAP: "This creature enters tapped" — static entering replacement; no
    // enters-tapped primitive is available for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        keywords: vec![KeywordAbility::Fading(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
