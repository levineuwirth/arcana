//! Crusading Knight — `{2}{W}{W}` 2/2 Human Knight.
//!
//! Oracle:
//! * Protection from black
//! * This creature gets +1/+1 for each Swamp your opponents control.
//!
//! Both lines are GAP'd:
//! * Protection is not a usable `KeywordAbility` variant.
//! * "+1/+1 for each Swamp your opponents control" is a static continuous
//!   self-pump with a dynamic board count — no expressible static primitive in
//!   this card class.
//!
//! Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crusading Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "Protection from black" — Protection is not a usable keyword.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
