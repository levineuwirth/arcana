//! Mothrider Cavalry — `{2}{W}{W}` 2/2 Human Samurai with Flying.
//! * "This spell costs {2} less to cast if you have no other creature
//!   cards in hand or if the only other creature cards in your hand are
//!   named Mothrider Cavalry." — static cost reduction; GAP.
//! * Flying — base characteristic.
//! * "Other creatures you control get +1/+1." — static anthem; GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mothrider Cavalry");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP (static): "This spell costs {2} less to cast if you have no
    // other creature cards in hand …" — conditional cost reduction.
    // GAP (static): "Other creatures you control get +1/+1." — anthem.
    reg.register(CardDefinition::new(name, chars))
}
