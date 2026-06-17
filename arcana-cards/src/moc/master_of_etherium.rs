//! Master of Etherium — `{2}{U}` */* Artifact Creature — Vedalken Wizard (U).
//! Both abilities are continuous statics with no triggered/activated component, so
//! both are GAP'd. Its */* CDA is unrepresentable in the Fixed-only P/T surface;
//! base P/T placeheld at 0/0.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Master of Etherium");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(wizard);

    // GAP: "Master of Etherium's power and toughness are each equal to the number of
    // artifacts you control." — characteristic-defining ability; PtValue surface here
    // is Fixed-only, so base P/T is placeheld at 0/0.
    // GAP: static anthem "Other artifact creatures you control get +1/+1."

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
