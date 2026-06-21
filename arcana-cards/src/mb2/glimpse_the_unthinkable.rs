//! Glimpse, the Unthinkable — `{2}{U}{B}` 4/5 Legendary Illusion Rogue.
//! Shroud.
//! "Glimpse, the Unthinkable can't be chosen."
//! "The name Glimpse, the Unthinkable can't be chosen."
//!
//! Shroud is wired. The two "can't be chosen" statics (an Un-set / specialty
//! mechanic) have no expressible representation — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glimpse, the Unthinkable");
    let illusion = reg.interner_mut().intern("Illusion");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Shroud],
        ..Default::default()
    };

    // GAP: static — "can't be chosen" / "the name can't be chosen". No
    // expressible representation for these specialty restrictions.
    reg.register(CardDefinition::new(name, chars))
}
