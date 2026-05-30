//! Geier Reach Bandit // Vildin-Pack Alpha — `{2}{R}` Human Rogue Werewolf 3/2.
//! Front face: Haste. At the beginning of each upkeep, if no spells were cast
//! last turn, transform this creature.
//! Back face (Vildin-Pack Alpha): Whenever a Werewolf you control enters, you
//! may transform it. At the beginning of each upkeep, if a player cast two or
//! more spells last turn, transform this creature.
//!
//! GAP: "No spells cast last turn" / "two or more spells last turn" werewolf
//! transform conditions are not modeled — day/night cycle not implemented.
//! The upkeep triggers are omitted; transform is not auto-wired.
//! GAP: Back-face "Whenever a Werewolf you control enters, you may transform it"
//! triggered ability not modeled (back-face-only triggered ability).
//! GAP: Back-face upkeep transform trigger not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Geier Reach Bandit");
    let human_sub = reg.interner_mut().intern("Human");
    let rogue_sub = reg.interner_mut().intern("Rogue");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(rogue_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        // GAP: front-face upkeep transform trigger (no-spells-last-turn) not modeled
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Vildin-Pack Alpha");
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
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
        // GAP: back-face-only triggered ability not modeled
        // (Whenever a Werewolf you control enters, you may transform it)
        // GAP: back-face upkeep transform trigger (two-spells-last-turn) not modeled
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
