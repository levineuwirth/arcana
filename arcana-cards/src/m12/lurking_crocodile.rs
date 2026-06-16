//! Lurking Crocodile — `{2}{G}` 2/2 green Crocodile.
//! "Bloodthirst 1 (If an opponent was dealt damage this turn, this creature
//!  enters with a +1/+1 counter on it.)
//!  Islandwalk."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lurking Crocodile");
    let crocodile = reg.interner_mut().intern("Crocodile");
    let island = reg.interner_mut().intern("Island");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crocodile);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Bloodthirst(1),
            KeywordAbility::Landwalk(island),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
