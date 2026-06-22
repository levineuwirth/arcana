//! Nighthawk Scavenger — `{1}{B}{B}` 1+*/3 black Vampire Rogue with
//! Flying, Deathtouch, Lifelink.
//!
//! Oracle:
//! * Flying, deathtouch, lifelink.
//! * Power is equal to 1 plus the number of card types among cards in your
//!   opponents' graveyards.
//!
//! The keyword line is fully wired. Power is transcribed as `StarPlus(1)`
//! (`1+*`); toughness is the fixed 3.
//!
//! GAP (CDA): "power is equal to 1 plus the number of card types among
//! cards in your opponents' graveyards" — counting distinct card types in
//! opponents' graveyards is not expressible with the documented `script`
//! surface, so the characteristic-defining computation is omitted. The
//! base `*+1` value is recorded.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nighthawk Scavenger");
    let vampire = reg.interner_mut().intern("Vampire");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::StarPlus(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Deathtouch,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
