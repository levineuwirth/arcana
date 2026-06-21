//! Juggernaut — `{4}` 5/3 colorless Artifact Creature — Juggernaut.
//!
//! Both printed abilities are GAP'd:
//!  * "This creature attacks each combat if able" — a must-attack
//!    self-static with no available primitive (Goad/ForbidAttacking are
//!    the only attack-requirement effects, and neither models a static
//!    self must-attack).
//!  * "This creature can't be blocked by Walls" — a subtype-restricted
//!    can't-be-blocked-by static with no available primitive.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Juggernaut");
    let juggernaut = reg.interner_mut().intern("Juggernaut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(juggernaut);

    // GAP: "attacks each combat if able" — static must-attack.
    // GAP: "can't be blocked by Walls" — subtype-restricted block static.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
