//! Rancorous Archaic — `{5}` 2/2 Avatar.
//! Trample, reach.
//! Converge — enters with a +1/+1 counter for each color of mana spent to cast it. (GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rancorous Archaic");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Reach],
        ..Default::default()
    };
    // GAP: Converge — "enters with a +1/+1 counter for each color of mana spent to cast it"
    // depends on the colors of mana used to pay the cost, which isn't a tracked quantity here.
    reg.register(CardDefinition::new(name, chars))
}
