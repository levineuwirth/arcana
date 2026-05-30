//! Solitary Hunter // One of the Pack — {3}{G} Creature — Human Warrior Werewolf // Werewolf (3/4)
//! Front: At the beginning of each upkeep, if no spells were cast last turn, transform this creature.
//! Back: At the beginning of each upkeep, if a player cast two or more spells last turn,
//!   transform this creature.
//! GAP: The exact werewolf day/night transform condition ("no spells cast last turn" /
//!   "cast two or more spells last turn") is not modeled — those are day/night cycle conditions.
//!   The card is registered with its back face declared but transform triggers are omitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Solitary Hunter");
    let human_sub = reg.interner_mut().intern("Human");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human_sub);
    front_subtypes.0.insert(warrior_sub);
    front_subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("One of the Pack");
    let werewolf_sub2 = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
        // GAP: Werewolf transform triggers (upkeep if no spells last turn / if 2+ spells last turn)
        // not modeled — day/night cycle conditions are deferred engine work.
    )
}
