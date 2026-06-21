//! Bolrac-Clan Basher — `{4}{R}{R}` 3/2 Cyclops Warrior with Double
//! strike and Trample.
//!
//! Oracle:
//! * Double strike, trample — keyword line.
//! * Disguise {3}{R}{R} — GAP: Disguise is not a modeled KeywordAbility
//!   (only the evergreen/parametrized list plus the `Warp` marker are
//!   available), and its face-down cast / turn-face-up mechanic is not
//!   expressible in this card class.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bolrac-Clan Basher");
    let cyclops = reg.interner_mut().intern("Cyclops");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyclops);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
