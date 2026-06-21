//! Mephidross Vampire — `{4}{B}{B}` 3/4 Vampire with Flying.
//!
//! Oracle:
//! * Flying — base keyword.
//! * Each creature you control is a Vampire in addition to its other creature
//!   types and has "Whenever this creature deals damage to a creature, put a
//!   +1/+1 counter on this creature." — both halves are STATIC continuous
//!   abilities (a type-grant + a granted-triggered-ability anthem). Neither is
//!   a triggered/activated ability on THIS card, and there is no board-wide
//!   continuous "grant type + ability to each creature you control" effect in
//!   the demonstrated API, so it is GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mephidross Vampire");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static — "Each creature you control is a Vampire in addition to its
    // other creature types and has '<granted triggered ability>'". A board-wide
    // continuous type-grant + ability-anthem over your creatures is not
    // expressible with the demonstrated API (no static-anthem effect; the
    // granted-ability primitive is per-target, not a continuous each-creature
    // anthem).
    reg.register(CardDefinition::new(name, chars))
}
