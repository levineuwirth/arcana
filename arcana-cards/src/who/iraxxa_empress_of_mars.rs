//! Iraxxa, Empress of Mars — `{2}{R}{R}` 5/4 Legendary Alien Warrior.
//! Trample; Battle cry.
//! Paradox — Whenever you cast a spell from anywhere other than your hand,
//! create a 2/2 red Alien Warrior creature token. (GAP — no "cast from a zone
//! other than hand" trigger condition.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iraxxa, Empress of Mars");
    let alien = reg.interner_mut().intern("Alien");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::BattleCry],
        ..Default::default()
    };

    // GAP: Paradox trigger — "Whenever you cast a spell from anywhere other than
    // your hand" has no matching TriggerCondition / SpellCast cast-zone filter.

    reg.register(CardDefinition::new(name, chars))
}
