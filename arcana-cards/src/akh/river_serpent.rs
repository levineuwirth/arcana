//! River Serpent — `{5}{U}` 5/5 Serpent with Cycling {U}.
//! "This creature can't attack unless there are five or more cards in your
//! graveyard." (static attack restriction — GAP'd below.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("River Serpent");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);
    // GAP: static "can't attack unless five or more cards in your graveyard"
    // is a continuous attack restriction, not a triggered/activated ability.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Cycling(ManaCost::parse("{U}").expect("valid cost"))],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
