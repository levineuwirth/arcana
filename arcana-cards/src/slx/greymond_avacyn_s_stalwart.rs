//! Greymond, Avacyn's Stalwart — `{2}{W}{W}` 3/4 Legendary Human
//! Soldier.
//! All three abilities are statics / ETB-choice and not expressible
//! as triggered or activated abilities, so they are GAP'd:
//!  * "As Greymond enters, choose two abilities from among first
//!    strike, vigilance, and lifelink." (as-enters choice)
//!  * "Humans you control have each of the chosen abilities." (static
//!    keyword-grant anthem)
//!  * "As long as you control four or more Humans, Humans you control
//!    get +2/+2." (conditional static anthem)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Greymond, Avacyn's Stalwart");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
