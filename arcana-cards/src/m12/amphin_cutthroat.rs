//! Amphin Cutthroat — vanilla 2/4 blue creature for `{3}{U}`. No
//! abilities; pure stats. A Salamander Rogue from the Magic 2015 set.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Amphin Cutthroat");
    let subtype_salamander = reg.interner_mut().intern("Salamander");
    let subtype_rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_salamander);
    subtypes.0.insert(subtype_rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
