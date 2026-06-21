//! Shorecrasher Elemental — `{U}{U}{U}` 3/3 Elemental.
//!
//! Oracle:
//! * {U}: Exile this creature, then return it to the battlefield face down
//!   under its owner's control.
//! * {1}: This creature gets +1/-1 or -1/+1 until end of turn.
//! * Megamorph {4}{U}
//!
//! All three lines are GAP'd:
//! * The {U} self-flicker returns the creature FACE DOWN — there is no
//!   face-down return primitive in the demonstrated surface.
//! * The {1} ability is a modal pump ("+1/-1 OR -1/+1"); activated abilities
//!   have no modal-choice mechanism in this surface and `Effect::Pump` is a
//!   single fixed delta, so the player's choice between the two splits is not
//!   expressible.
//! * Megamorph is not a usable `KeywordAbility` variant.
//!
//! Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shorecrasher Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
