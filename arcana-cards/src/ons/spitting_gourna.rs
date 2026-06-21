//! Spitting Gourna — `{3}{G}{G}` 3/4 Creature — Beast.
//!
//! Oracle:
//! * Reach.
//! * Morph {4}{G}. (GAP: Morph is a face-down casting mechanic with no
//!   demonstrated API; only the Reach keyword is modeled.)
//!
//! Decomposition: Reach → `keywords`; Morph is GAP'd (cast-face-down
//! mechanic, not a triggered/activated ability).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spitting Gourna");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: "Morph {4}{G}" — cast face down as a 2/2 for {3}, turn face up for
    // its morph cost. No Morph / face-down casting mechanic in the API.

    reg.register(CardDefinition::new(name, chars))
}
