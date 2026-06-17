//! Chameleon Spirit — `{3}{U}` */* blue Illusion Spirit.
//! As it enters, choose a color; its power and toughness each equal the number
//! of permanents of the chosen color your opponents control.
//! GAP: "choose a color as this enters" plus a characteristic-defining P/T from
//! that chosen color is not expressible (no color-choice enters effect, no CDA
//! mechanism). P/T set to 0/0 as a placeholder; no abilities emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chameleon Spirit");
    let illusion = reg.interner_mut().intern("Illusion");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
