//! Skarrgan Pit-Skulk — `{G}` 1/1 Human Warrior with Bloodthirst 1.
//! "Creatures with power less than this creature's power can't block it."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skarrgan Pit-Skulk");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Bloodthirst(1)],
        ..Default::default()
    };

    // GAP: "Creatures with power less than this creature's power can't block
    // it" is a static skulk-like blocking restriction, not a
    // triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
