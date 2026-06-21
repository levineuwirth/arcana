//! Urabrask the Hidden — `{3}{R}{R}` 4/4 Legendary Phyrexian Praetor.
//!
//! * Creatures you control have haste. (Static keyword-granting anthem
//!   — GAP'd: no triggered/activated form.)
//! * Creatures your opponents control enter tapped. (Static
//!   enters-tapped replacement on opponents' creatures — GAP'd.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urabrask the Hidden");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let praetor = reg.interner_mut().intern("Praetor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(praetor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Creatures you control have haste" — anthem-style
    // continuous keyword grant.
    // GAP: static "Creatures your opponents control enter tapped" —
    // enters-tapped replacement scoped to opponents.

    reg.register(CardDefinition::new(name, chars))
}
