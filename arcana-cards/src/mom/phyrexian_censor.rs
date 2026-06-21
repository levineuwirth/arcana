//! Phyrexian Censor — `{2}{W}` 3/3 Phyrexian Wizard.
//! "Each player can't cast more than one non-Phyrexian spell each turn.
//!  Non-Phyrexian creatures enter tapped."
//!
//! Both lines are pure continuous statics (no trigger word, no cost)
//! with no usable hook in this card class — emitted as bones only with
//! GAP notes.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Censor");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(wizard);

    // GAP (static): "Each player can't cast more than one non-Phyrexian spell
    // each turn" — cast-restriction static, no usable hook.
    // GAP (static): "Non-Phyrexian creatures enter tapped" — replacement-style
    // static, no usable hook in this card class.
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
