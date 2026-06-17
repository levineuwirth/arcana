//! A-Cosmos Charger — `{2}{U}` 3/2 Horse Spirit with Flash and Flying.
//! "Foretelling cards from your hand costs {1} less and can be done on any
//! player's turn." and "Foretell {U}".

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Cosmos Charger");
    let horse = reg.interner_mut().intern("Horse");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Foretelling cards from your hand costs {1} less and can be done on
    // any player's turn" — a cost-reduction/timing-permission static affecting
    // the Foretell action is not expressible.
    // GAP: "Foretell {U}" — Foretell is not a supported KeywordAbility variant.
    reg.register(CardDefinition::new(name, chars))
}
