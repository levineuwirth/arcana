//! Boldwyr Aggressor — `{3}{R}{R}` 2/5 Giant Warrior with Double strike.
//! "Other Giants you control have double strike."
//!
//! The Giant-anthem static (granting double strike to other Giants) is
//! a continuous static ability with no demonstrated primitive and is
//! GAP'd; only the creature's own Double strike keyword is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boldwyr Aggressor");
    let giant = reg.interner_mut().intern("Giant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // GAP: static "Other Giants you control have double strike" — a
    // continuous anthem with no demonstrated primitive.
    reg.register(CardDefinition::new(name, chars))
}
