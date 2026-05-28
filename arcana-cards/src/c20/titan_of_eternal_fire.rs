//! Titan of Eternal Fire — `{5}{R}` 5/6 red Giant.
//! "Each Human creature you control has '{R}, {T}: This creature deals 1
//! damage to any target.'"
//! GAP: granting activated abilities to other permanents is a static ability
//! not expressible in the trigger/activation framework; emitting as bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Titan of Eternal Fire");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    // GAP: "each Human creature you control has '{R},{T}: deals 1 damage to
    // any target'" — static ability granting is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
