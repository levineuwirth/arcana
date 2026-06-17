//! Cosmos Charger — `{3}{U}` 3/3 Horse Spirit with Flash and Flying.
//! Foretelling cards from your hand costs {1} less and can be done on any
//! player's turn. Foretell {2}{U}.
//!
//! Foretell is not in the usable keyword surface, and the foretell-cost
//! static is not expressible — only Flash and Flying are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cosmos Charger");
    let horse = reg.interner_mut().intern("Horse");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    subtypes.0.insert(spirit);

    // GAP: Foretell {2}{U} keyword — not in the usable keyword surface.
    // GAP: foretell cost-reduction / any-turn static — not expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
