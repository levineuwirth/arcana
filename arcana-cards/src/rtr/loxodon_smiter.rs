//! Loxodon Smiter — `{1}{G}{W}` 4/4 Elephant Soldier.
//! "This spell can't be countered." (GAP — uncounterable static not expressible)
//! "If a spell or ability an opponent controls causes you to discard this card, put it
//! onto the battlefield instead of putting it into your graveyard." (GAP — discard
//! replacement effect not expressible)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Loxodon Smiter");
    let elephant = reg.interner_mut().intern("Elephant");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "This spell can't be countered." — uncounterable static.
    // GAP: opponent-caused-discard → put onto battlefield replacement effect.
    reg.register(CardDefinition::new(name, chars))
}
