//! Donatello, the Brains — `{2}{U}` 2/4 Legendary Mutant Ninja Turtle.
//! If one or more tokens would be created under your control, those tokens
//! plus a Mutagen token are created instead.
//! Partner—Character select.
//!
//! The token-creation replacement effect is not expressible as a
//! triggered/activated ability, and Partner is not in the usable keyword
//! surface — only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Donatello, the Brains");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);

    // GAP: token-creation replacement effect ("those tokens plus a Mutagen
    //      token are created instead") is not a triggered/activated ability.
    // GAP: Partner—Character select is not in the usable keyword surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
