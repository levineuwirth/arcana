//! Chittering Illuminator — `{G}{G}` 2/2 Enchantment Creature —
//! Squirrel Glimmer.
//!
//! * As long as Chittering Illuminator is at the top of your library,
//!   you may look at it any time and you may cast it.
//! * As long as the top card of your library is a creature card, you
//!   may look at it any time and you may cast it.
//!
//! Both lines are pure static "play from the top of your library"
//! permissions — there is no expressible primitive for top-of-library
//! casting permission. Both are GAP'd; the card carries only its bones.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chittering Illuminator");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let glimmer = reg.interner_mut().intern("Glimmer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(glimmer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static "you may look at / cast this from the top of your
    // library" — no expressible top-of-library play-permission primitive.
    // GAP: static "you may look at / cast the top card while it's a
    // creature card" — same.

    reg.register(CardDefinition::new(name, chars))
}
