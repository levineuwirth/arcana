//! Intellect Devourer — `{3}{B}` 2/4 black Horror.
//!
//! * "Devour Intellect — When this creature enters, each opponent exiles a card
//!   from their hand until this creature leaves the battlefield." — GAP: an ETB
//!   trigger where each opponent CHOOSES a card from their HAND to exile, with a
//!   "until this leaves" return linkage. `ExileUntilSourceLeaves` works on a
//!   known battlefield target, not a per-opponent hand-card choice, and there
//!   is no exile-card-from-hand-until-leaves primitive — unexpressible.
//! * "Body Thief — You may play lands and cast spells from among cards exiled
//!   with this creature. If you cast a spell this way, you may spend mana as
//!   though it were mana of any color to cast it." — GAP: a continuous
//!   play-permission static over exiled cards; not a triggered/activated
//!   ability.
//!
//! Neither ability is expressible; only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Intellect Devourer");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
