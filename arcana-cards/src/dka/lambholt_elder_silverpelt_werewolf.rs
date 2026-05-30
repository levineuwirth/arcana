//! Lambholt Elder // Silverpelt Werewolf — `{2}{G}` Human Werewolf 1/2.
//! Front face: At the beginning of each upkeep, if no spells were cast last
//! turn, transform this creature.
//! Back face (Silverpelt Werewolf): Whenever this creature deals combat damage
//! to a player, draw a card. At the beginning of each upkeep, if a player cast
//! two or more spells last turn, transform this creature.
//!
//! GAP: "No spells cast last turn" / "two or more spells last turn" werewolf
//! transform conditions not modeled — day/night cycle not implemented.
//! The upkeep transform triggers are omitted.
//! GAP: Back-face "Whenever this creature deals combat damage to a player, draw
//! a card" triggered ability not modeled (back-face-only triggered ability).
//! GAP: Back-face upkeep transform trigger not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lambholt Elder");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: front-face upkeep transform trigger (no-spells-last-turn) not modeled
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Silverpelt Werewolf");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(5)),
            // GAP: back-face-only triggered ability not modeled
            // (Whenever this creature deals combat damage to a player, draw a card)
            // GAP: back-face upkeep transform trigger not modeled
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
