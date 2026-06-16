//! Sycorax Commander — `{2}{B}{R}` 4/2 Alien Soldier with First strike
//! and Haste.
//!
//! "Sanctified Rules of Combat" is a flavor-keyword Scryfall parses but
//! is not in the usable keyword surface (omitted). The ETB villainous
//! choice (each opponent chooses to discard-and-redraw-minus-one OR
//! take damage equal to their hand size) is a multi-branch
//! opponent-choice with no demonstrated wrapper — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sycorax Commander");
    let alien = reg.interner_mut().intern("Alien");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: ETB villainous choice (each opponent chooses to discard their
    // hand and draw that many minus one, OR be dealt damage equal to
    // their hand size) — multi-branch opponent-choice not expressible.
    reg.register(CardDefinition::new(name, chars))
}
