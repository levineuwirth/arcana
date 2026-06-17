//! Fumiko the Lowblood — `{2}{R}{R}` 3/2 Legendary Human Samurai.
//! "Fumiko has bushido X, where X is the number of attacking creatures.
//!  Creatures your opponents control attack each combat if able."
//!
//! Bushido is a parametrized keyword taking a FIXED u8 — "bushido X where X is
//! the number of attacking creatures" is a dynamic value the keyword can't
//! carry, so it is GAP'd rather than misrepresented as a fixed Bushido(N).
//! The "opponents' creatures attack each combat if able" line is a static
//! attack-forcing rule with no expressible primitive.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fumiko the Lowblood");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "bushido X, where X is the number of attacking creatures" — Bushido(N) takes a fixed u8, not a dynamic X.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Creatures your opponents control attack each combat if able" — no attack-forcing primitive.
    reg.register(CardDefinition::new(name, chars))
}
