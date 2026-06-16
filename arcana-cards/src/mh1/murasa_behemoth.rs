//! Murasa Behemoth — `{4}{G}{G}` 5/5 Beast with Trample.
//! Gets +3/+3 as long as there is a land card in your graveyard
//!   (conditional continuous static — GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Murasa Behemoth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    // GAP (static): "This creature gets +3/+3 as long as there is a land card in
    //   your graveyard" — a self-referential conditional continuous buff has no
    //   primitive in this card class.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
