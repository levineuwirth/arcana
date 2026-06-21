//! Sewer Shambler — `{2}{B}` 2/1 Zombie.
//! Swampwalk.
//! Scavenge {2}{B}.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sewer Shambler");
    let zombie = reg.interner_mut().intern("Zombie");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::Landwalk(swamp),
            KeywordAbility::Scavenge(ManaCost::parse("{2}{B}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
