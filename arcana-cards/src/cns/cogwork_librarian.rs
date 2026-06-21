//! Cogwork Librarian — `{4}` 3/3 Artifact Creature — Construct.
//! "Draft this card face up."
//! "As you draft a card, you may draft an additional card from that
//! booster pack. If you do, put this card into that booster pack."
//!
//! Both lines are draft-environment mechanics (CR 707 draft matters),
//! not in-game triggered/activated/static abilities. The engine models
//! gameplay, not the draft, so both are GAP'd; only the bones remain.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cogwork Librarian");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Draft this card face up." — a draft-environment mechanic, no
    // in-game effect.
    // GAP: "As you draft a card, you may draft an additional card ... put this
    // card into that booster pack." — a draft-environment mechanic, no in-game
    // effect.

    reg.register(CardDefinition::new(name, chars))
}
