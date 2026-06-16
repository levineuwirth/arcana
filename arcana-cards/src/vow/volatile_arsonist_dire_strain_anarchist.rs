//! Volatile Arsonist // Dire-Strain Anarchist
//!
//! Front face: Creature — Human Werewolf, {3}{R}{R}, 4/4, Menace, Haste.
//! Whenever this creature attacks, it deals 1 damage to each of up to one
//! target creature, up to one target player, and/or up to one target planeswalker.
//!
//! Back face: Creature — Werewolf, 5/5, Menace, Haste.
//! Whenever this creature attacks, it deals 2 damage to each of up to one
//! target creature, up to one target player, and/or up to one target planeswalker.
//!
//! GAP: Daybound / Nightbound day/night cycle not modeled; transform is not wired.
//! GAP: Back-face-only triggered ability (attacks, deal 2 damage to targets) not modeled.
//! GAP: "up to one target creature, up to one target player, and/or up to one target
//!       planeswalker" multi-target attack trigger is not expressible with a single
//!       TargetRequirement shape; omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Volatile Arsonist");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: attack trigger (deal 1 damage to up to one target creature / player / planeswalker)
    // not expressible — multi-type "up to one each" target shape not modeled.

    let back_name = reg.interner_mut().intern("Dire-Strain Anarchist");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Menace, KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
    // GAP: Daybound/Nightbound transform triggers not modeled.
    // GAP: Back-face attack trigger (deal 2 damage to up to one target creature/player/planeswalker) not modeled.
}
