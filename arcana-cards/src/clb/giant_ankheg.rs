//! Giant Ankheg — `{6}{G}{G}` 8/8 Insect with Trample and Ward {2}.
//!
//! "Other creatures you control have trample and ward {2}." is a static
//! continuous keyword-granting ability with no triggered/activated
//! shape, so it is GAP'd; the card's own Trample and Ward {2} are base
//! characteristics.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Ankheg");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    // GAP: static "Other creatures you control have trample and ward {2}"
    // is a continuous keyword-granting ability with no triggered/activated
    // wiring available in this card class.

    reg.register(CardDefinition::new(name, chars))
}
