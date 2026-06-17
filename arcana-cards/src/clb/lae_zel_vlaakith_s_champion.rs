//! Lae'zel, Vlaakith's Champion — `{2}{W}` 3/3 Legendary Gith Warrior.
//!
//! GAP: "If you would put one or more counters on a creature or planeswalker you
//! control or on yourself, put that many plus one of each of those kinds
//! instead" is a counter-doubling replacement effect; there is no
//! replacement-effect-installing primitive available to a creature card here.
//! GAP: "Choose a Background" is not among the supported KeywordAbility variants
//! (a deck-construction / commander-zone rule), so it is omitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lae'zel, Vlaakith's Champion");
    let gith = reg.interner_mut().intern("Gith");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gith);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
