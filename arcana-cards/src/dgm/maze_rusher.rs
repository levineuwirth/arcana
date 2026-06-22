//! Maze Rusher — `{5}{R}` 6/3 Elemental with Haste.
//! "Multicolored creatures you control have haste."
//!
//! Haste is wired. The static "multicolored creatures you control have
//! haste" is a continuous keyword-granting static with no trigger or
//! activation cost; it is not expressible as a triggered/activated
//! ability and is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "Multicolored creatures you control have haste." — a static
// continuous keyword-granting ability with no trigger/cost; not
// expressible in the MultiAbilityCreature surface.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maze Rusher");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
