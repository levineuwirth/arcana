//! Avatar of Will — `{6}{U}{U}` 5/6 Avatar with Flying.
//! "If an opponent has no cards in hand, this spell costs {6} less to cast.
//!  Flying"
//!
//! Flying is a base keyword. The conditional cast-cost reduction is not a
//! modeled effect — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avatar of Will");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "If an opponent has no cards in hand, this spell costs {6} less" —
    // conditional cast-cost reduction, not modeled.
    reg.register(CardDefinition::new(name, chars))
}
