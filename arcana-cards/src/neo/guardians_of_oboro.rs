//! Guardians of Oboro — `{2}{U}` 3/4 Moonfolk Samurai with Defender.
//! "Modified creatures you control can attack as though they didn't have
//!  defender."
//!
//! GAP: the "modified creatures you control can attack as though they
//! didn't have defender" static continuous ability has no trigger or cost
//! and grants a combat permission to other creatures — not expressible as
//! a triggered/activated ability; omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guardians of Oboro");
    let moonfolk = reg.interner_mut().intern("Moonfolk");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moonfolk);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
