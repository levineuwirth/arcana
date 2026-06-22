//! Reverse Ninja — `{1}{B}` 1/1 black Snake Ninja.
//!
//! * Deathtouch (keyword).
//! * Ustujnin {B} ("{B}, Return a blocking creature you control to
//!   hand: Put this card onto the battlefield from your hand blocking
//!   the same creature.") — both the "return a blocking creature you
//!   control" cost and the "put onto the battlefield blocking the same
//!   creature" effect have no expressible primitive on this shape; the
//!   whole ability is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reverse Ninja");
    let snake = reg.interner_mut().intern("Snake");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(ninja);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };
    // GAP: Ustujnin {B} — cast-from-hand-into-blocking mechanic; neither
    // the "return a blocking creature you control" cost nor the "put
    // onto the battlefield blocking the same creature" effect is
    // expressible on this shape.
    reg.register(CardDefinition::new(name, chars))
}
