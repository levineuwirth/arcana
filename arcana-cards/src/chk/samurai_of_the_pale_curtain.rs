//! Samurai of the Pale Curtain — `{W}{W}` 2/2 white Fox Samurai.
//! Bushido 1.
//! If a permanent would be put into a graveyard, exile it instead.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Samurai of the Pale Curtain");
    let fox = reg.interner_mut().intern("Fox");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Bushido(1)],
        // GAP: "If a permanent would be put into a graveyard, exile it instead."
        // A board-wide death->exile replacement static; no demonstrated
        // replacement-effect primitive expresses it.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
