//! Wingrattle Scarecrow — `{3}` 2/2 colorless Artifact Creature —
//! Scarecrow.
//!
//! Rules text (both lines are conditional statics with no expressible
//! primitive — emitted as bones only):
//! * "This creature has flying as long as you control a blue
//!   creature." — a condition-gated keyword grant. GAP'd: no
//!   "has <keyword> as long as <condition>" static primitive.
//! * "This creature has persist as long as you control a black
//!   creature." — same shape. GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wingrattle Scarecrow");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: conditional static "has flying as long as you control a blue creature".
    // GAP: conditional static "has persist as long as you control a black creature".
    reg.register(CardDefinition::new(name, chars))
}
