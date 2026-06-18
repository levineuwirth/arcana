//! Multani, Maro-Sorcerer — `{4}{G}{G}` */* Legendary Elemental Sorcerer
//! with Shroud. Its power and toughness are each equal to the total number
//! of cards in all players' hands — a characteristic-defining ability whose
//! computation isn't expressible with the demonstrated API, so the */* is
//! recorded as PtValue::Star and the CDA itself is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Multani, Maro-Sorcerer");
    let elemental = reg.interner_mut().intern("Elemental");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(sorcerer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Shroud],
        ..Default::default()
    };

    // GAP: CDA "power and toughness each equal to the total number of cards
    // in all players' hands" — characteristic-defining P/T not expressible
    // here; */* recorded via PtValue::Star.
    reg.register(CardDefinition::new(name, chars))
}
