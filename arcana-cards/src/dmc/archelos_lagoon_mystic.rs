//! Archelos, Lagoon Mystic — `{1}{B}{G}{U}` 2/4 Legendary Turtle Shaman.
//! "As long as Archelos is tapped, other permanents enter tapped."
//! "As long as Archelos is untapped, other permanents enter untapped."
//!
//! Both lines are static replacement effects (modifying how OTHER
//! permanents enter, gated on this creature's tapped state) with no
//! triggered/activated ability shape, so both are GAP'd. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archelos, Lagoon Mystic");
    let turtle = reg.interner_mut().intern("Turtle");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "as long as Archelos is tapped, other permanents enter tapped"
    // and the untapped mirror — static enters-tapped replacement effects,
    // no triggered/activated ability shape.

    reg.register(CardDefinition::new(name, chars))
}
