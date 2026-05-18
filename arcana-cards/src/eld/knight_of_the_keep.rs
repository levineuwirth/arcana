//! Knight of the Keep — vanilla 3/2 white creature for `{2}{W}`. Creature — Human Knight.
//! No abilities; pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of the Keep");
    let subtype_human = reg.interner_mut().intern("Human");
    let subtype_knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_human);
    subtypes.0.insert(subtype_knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
