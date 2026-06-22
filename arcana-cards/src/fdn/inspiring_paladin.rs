//! Inspiring Paladin — `{2}{W}` 3/3 white Human Knight.
//!
//! Oracle text:
//! * During your turn, this creature has first strike.
//! * During your turn, creatures you control with +1/+1 counters on
//!   them have first strike.
//!
//! Implemented: the bones only (3/3 Human Knight).
//!
//! GAP: both lines are turn-conditional static continuous abilities
//! ("during your turn, … has first strike") — neither a keyword on the
//! base characteristics (the grant is conditional on the active player)
//! nor any triggered/activated ability, and the engine exposes no
//! conditional-static representation here, so both are omitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inspiring Paladin");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
