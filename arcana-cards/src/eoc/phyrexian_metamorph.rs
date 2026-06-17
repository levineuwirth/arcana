//! Phyrexian Metamorph — `{3}{U/P}` 0/0 Artifact Creature — Phyrexian
//! Shapeshifter (blue). "You may have this creature enter as a copy of any
//! artifact or creature on the battlefield, except it's an artifact in
//! addition to its other types."
//!
//! The clone-on-enter replacement (a copy-as-enters effect with a +artifact
//! rider) is not expressible with the demonstrated API — there is no
//! enters-as-a-copy replacement Effect, and Effect::CopyPermanent mints a
//! NEW token rather than altering how THIS creature enters. So no abilities
//! are wired; only the bones (including the {U/P} Phyrexian pip in the cost)
//! are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Metamorph");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U/P}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "enter as a copy of any artifact or creature ... it's an artifact in
    // addition to its other types" — a copy-as-enters replacement effect is not
    // expressible (CopyPermanent mints a token; no enters-as-copy variant).
    reg.register(CardDefinition::new(name, chars))
}
