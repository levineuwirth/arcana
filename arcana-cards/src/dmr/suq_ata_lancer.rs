//! Suq'Ata Lancer — `{2}{R}` 2/2 Human Knight.
//!
//! Haste.
//! Flanking (Whenever a creature without flanking blocks this creature, the
//! blocking creature gets -1/-1 until end of turn.)
//!
//! Both are base keywords; nothing beyond listing them is required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Suq'Ata Lancer");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flanking, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
