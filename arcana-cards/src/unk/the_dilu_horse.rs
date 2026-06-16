//! The Dilu Horse — `{2}{G}` 2/2 Legendary Horse with Horsemanship.
//! "Commanders you control have horsemanship. Partner."
//!
//! Only Horsemanship is expressible. Partner is not a supported KeywordAbility
//! variant, and the "Commanders you control have horsemanship" static (a
//! commander-scoped continuous keyword grant) has no representation here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Dilu Horse");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Horsemanship],
        // GAP: Partner — not a supported KeywordAbility variant.
        // GAP: "Commanders you control have horsemanship" — commander-scoped
        // continuous keyword grant has no representation in the demonstrated API.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
