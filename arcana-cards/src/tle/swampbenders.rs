//! Swampbenders — `{4}{G}{G}` */* Human Druid Ally.
//! "Swampbenders's power and toughness are each equal to the number of Swamps
//! on the battlefield." (CDA static — GAP)
//! "Lands you control are Swamps in addition to their other types." (static — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Swampbenders");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    // GAP: characteristic-defining ability — P/T equal to the number of Swamps
    // on the battlefield is a continuous CDA, not a triggered/activated ability.
    // GAP: static — "Lands you control are Swamps in addition to their other
    // types" is a continuous type-adding ability.
    reg.register(CardDefinition::new(name, chars))
}
