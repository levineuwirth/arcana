//! Markov Crusader — `{4}{B}` 4/3 Vampire Knight with Lifelink.
//!
//! Oracle:
//! * Lifelink
//! * This creature has haste as long as you control another Vampire.
//!
//! Lifelink is a base keyword. The conditional-haste line is a pure static
//! continuous ability gated on a board state ("as long as you control another
//! Vampire") — not a triggered or activated ability — so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Markov Crusader");
    let vampire = reg.interner_mut().intern("Vampire");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: "has haste as long as you control another Vampire" — a conditional
    // static keyword grant, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
