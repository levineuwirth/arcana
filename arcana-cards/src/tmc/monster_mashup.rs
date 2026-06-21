//! Monster Mashup — `{3}{R}` 4/3 Creature — Werewolf Fish Zombie Vampire.
//! Reach, Menace. Both are evergreen base-characteristic keywords; nothing
//! beyond listing them is required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Monster Mashup");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let fish = reg.interner_mut().intern("Fish");
    let zombie = reg.interner_mut().intern("Zombie");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(werewolf);
    subtypes.0.insert(fish);
    subtypes.0.insert(zombie);
    subtypes.0.insert(vampire);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Menace],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
