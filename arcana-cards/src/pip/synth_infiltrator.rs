//! Synth Infiltrator — `{3}{U}{U}` 0/0 blue Artifact Creature — Synth.
//! "Improvise. You may have this creature enter as a copy of any creature on
//! the battlefield, except it's a Synth artifact creature in addition to its
//! other types."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Synth Infiltrator");
    let synth = reg.interner_mut().intern("Synth");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(synth);
    // GAP: "Improvise" — not in the supported KeywordAbility set (artifacts
    // help cast cost reduction is not a cost primitive).
    // GAP: "You may have this creature enter as a copy of any creature on the
    // battlefield ..." — an enter-as-a-copy clone replacement effect is not
    // expressible in this shape (no copy-on-ETB primitive).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
