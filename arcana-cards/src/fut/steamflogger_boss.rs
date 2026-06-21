//! Steamflogger Boss — `{3}{R}` 3/3 red Goblin Rigger.
//!
//! * Assemble — not in the usable keyword surface, GAP'd.
//! * "Other Riggers you control get +1/+0 and have haste." A static
//!   continuous anthem; static continuous abilities are not expressible
//!   in the triggered/activated decomposition surface, GAP'd.
//! * "If a Rigger you control would assemble a Contraption, it assembles
//!   two Contraptions instead." A replacement effect over Contraption
//!   assembly, which is not modeled, GAP'd.
//!
//! Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Steamflogger Boss");
    let goblin = reg.interner_mut().intern("Goblin");
    let rigger = reg.interner_mut().intern("Rigger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(rigger);

    // GAP: Assemble keyword not usable; static anthem "Other Riggers …
    // get +1/+0 and have haste" not expressible; Contraption-assembly
    // replacement not modeled.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
