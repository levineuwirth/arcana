//! Oathsworn Vampire — `{1}{B}` 2/2 Vampire Knight.
//! "This creature enters tapped. You may cast this card from your graveyard
//! if you gained life this turn."
//!
//! Both lines are static permissions / replacement-style rules that have no
//! triggered or activated ability surface in the demonstrated API:
//! - "enters tapped" is an entry replacement (no `Effect`/trigger for it here).
//! - "you may cast from your graveyard if you gained life" is an alternative
//!   casting permission, not an activated/triggered ability.
//! Both are GAP'd; the card body carries only its bones.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oathsworn Vampire");
    let vampire = reg.interner_mut().intern("Vampire");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "This creature enters tapped." — entry replacement, no API surface.
    // GAP: "You may cast this card from your graveyard if you gained life this
    //       turn." — alternative casting permission, not an activated/triggered
    //       ability expressible here.
    reg.register(CardDefinition::new(name, chars))
}
