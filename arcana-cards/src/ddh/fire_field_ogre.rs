//! Fire-Field Ogre — `{1}{U}{B}{R}` 4/2 Ogre Mutant with First strike.
//!
//! Oracle:
//! * First strike (keyword).
//! * Unearth {U}{B}{R} (return from graveyard to the battlefield, gains haste,
//!   exiled at the next end step or if it would leave). (GAP — Unearth is not
//!   in the expressible keyword surface and has no activated-from-graveyard
//!   keyword representation here.)
//!
//! Only First strike is expressible. Unearth would be a graveyard-activated
//! self-reanimation with a delayed exile rider; there is no `KeywordAbility`
//! variant for it, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fire-Field Ogre");
    let ogre = reg.interner_mut().intern("Ogre");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(mutant);

    // GAP: "Unearth {U}{B}{R}" — graveyard-activated self-reanimation keyword
    // with a delayed exile rider; no expressible KeywordAbility variant.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
