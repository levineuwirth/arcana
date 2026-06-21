//! Orthodoxy Enforcer — `{3}{W}` 2/4 white Phyrexian Cleric.
//!
//! Vigilance.
//! This creature gets +2/+0 as long as you control two or more
//! artifacts.
//!
//! The +2/+0 conditional static buff is a continuous (layer 7c)
//! self-pump gated on a board count; there is no triggered/activated
//! ability shape for it in the demonstrated API, so it is GAP'd. The
//! Vigilance keyword is fully expressed.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Orthodoxy Enforcer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "gets +2/+0 as long as you control two or more artifacts"
    // is a conditional continuous self-buff with no triggered/activated
    // shape in the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
