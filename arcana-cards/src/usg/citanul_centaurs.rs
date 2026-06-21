//! Citanul Centaurs — `{3}{G}` 6/3 Centaur with Shroud.
//!
//! Echo {3}{G} is not part of the usable keyword surface (no
//! `KeywordAbility::Echo`), and the echo upkeep payment is not
//! expressible with the available activation/trigger primitives, so it
//! is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Citanul Centaurs");
    let centaur = reg.interner_mut().intern("Centaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Shroud],
        ..Default::default()
    };

    // GAP: Echo {3}{G} — no KeywordAbility::Echo and the "sacrifice unless
    // you pay its echo cost at your upkeep" mechanic is not expressible.

    reg.register(CardDefinition::new(name, chars))
}
