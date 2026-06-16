//! Dungrove Elder — `{2}{G}` */* Treefolk with Hexproof.
//! "Dungrove Elder's power and toughness are each equal to the number of
//! Forests you control."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dungrove Elder");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    // GAP: the characteristic-defining ability "power and toughness are each
    // equal to the number of Forests you control" sets the `*` value; no CDA
    // registration hook is demonstrated for this shape, so P/T is left as `*`
    // (PtValue::Star) without the computing static.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
