//! Aven Sunstriker — `{1}{W}{W}` 1/1 Bird Warrior with Flying, Double strike.
//!
//! Oracle:
//! * Flying.
//! * Double strike.
//! * Megamorph {4}{W}.
//!
//! Flying and Double strike are base characteristics. Megamorph is GAP'd: it
//! is a face-down casting alternative (`{3}` for a 2/2, turn face up for
//! {4}{W} with a +1/+1 counter) that is not in the usable keyword surface and
//! has no triggered/activated ability to attach.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aven Sunstriker");
    let bird = reg.interner_mut().intern("Bird");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // GAP: Megamorph {4}{W} — face-down casting alternative not in the usable
    // keyword surface; no expressible ability to attach.
    reg.register(CardDefinition::new(name, chars))
}
