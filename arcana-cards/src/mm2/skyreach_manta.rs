//! Skyreach Manta — `{5}` 0/0 Artifact Creature — Fish with Sunburst and Flying.
//! "Sunburst (This creature enters with a +1/+1 counter on it for each color of mana
//!  spent to cast it.)
//!  Flying"
//!
//! Both abilities are base keywords; nothing beyond listing them is required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyreach Manta");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Sunburst, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
