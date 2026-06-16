//! Visage Bandit — `{3}{U}` 2/2 Shapeshifter Rogue.
//! "You may have this creature enter as a copy of a creature you control,
//! except it's a Shapeshifter Rogue in addition to its other types.
//! Plot {2}{U}."
//!
//! Both abilities are GAP'd: the "enter as a copy" replacement effect is a
//! static/replacement with no triggered/activated form and no copy-as-enters
//! primitive, and Plot is not a KeywordAbility variant. Only the bones are
//! emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Visage Bandit");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "enter as a copy of a creature you control" — copy-as-it-enters
    // replacement effect, no primitive.
    // GAP: "Plot {2}{U}" — Plot is not a KeywordAbility variant.
    reg.register(CardDefinition::new(name, chars))
}
