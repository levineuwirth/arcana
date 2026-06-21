//! Serra's Emissary — `{4}{W}{W}{W}` 7/7 white Angel with Flying.
//!
//! * Flying — keyword line.
//! * GAP: "As this creature enters, choose a card type." — an
//!   as-enters choice with no triggered/activated representation (no
//!   choose-a-card-type effect).
//! * GAP: "You and creatures you control have protection from the chosen
//!   card type." — a board-wide static continuous grant of a chosen-type
//!   protection; not expressible in the MultiAbilityCreature shape (no
//!   static board-wide protection primitive, and protection-from-a-
//!   chosen-card-type has no effect variant).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serra's Emissary");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
