//! Tarkir Duneshaper // Burnished Dunestomper
//!
//! Front: {W} Creature — Dog Warrior 1/2.
//! Activated ability: {4}{G/P}: Transform this creature. Activate only as a sorcery.
//! ({G/P} can be paid with either {G} or 2 life.)
//!
//! Back: Creature — Phyrexian Dog Warrior. Has Trample (static keyword on back face).
//!
//! GAP: {G/P} hybrid-phyrexian mana cost is not expressible as a ManaCost::parse string
//! in this engine (no {G/P} hybrid symbol support). The transform activation is omitted;
//! only the static characteristics are wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tarkir Duneshaper");
    let sub_dog = reg.interner_mut().intern("Dog");
    let sub_warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_dog);
    subtypes.0.insert(sub_warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Burnished Dunestomper");
    let sub_phyrexian = reg.interner_mut().intern("Phyrexian");
    let sub_dog_b = reg.interner_mut().intern("Dog");
    let sub_warrior_b = reg.interner_mut().intern("Warrior");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(sub_phyrexian);
    back_subtypes.0.insert(sub_dog_b);
    back_subtypes.0.insert(sub_warrior_b);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: The transform activation cost {4}{G/P} uses a hybrid-phyrexian mana symbol
    // not expressible via ManaCost::parse. The activation is not modeled.
    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
