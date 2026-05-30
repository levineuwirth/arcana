//! Oakshade Stalker // Moonlit Ambusher
//!
//! Front face: Creature — Human Ranger Werewolf {2}{G}, 3/3, green.
//! "You may cast this spell as though it had flash if you pay {2} more."
//! (GAP: alternate-cost flash-if-pay-more cast mode not modeled in engine.)
//! Daybound (GAP: day/night cycle not modeled).
//!
//! Back face: Creature — Werewolf, colors green.
//! Nightbound (GAP: day/night cycle not modeled).
//!
//! Transform triggered abilities are GAP'd since the precise day/night
//! werewolf trigger conditions are not available in the engine.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oakshade Stalker");
    let human_sub = reg.interner_mut().intern("Human");
    let ranger_sub = reg.interner_mut().intern("Ranger");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(ranger_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Moonlit Ambusher");
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
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
        // GAP: Daybound/Nightbound day-night cycle transform triggers not modeled
        // GAP: alternate-cost flash-if-pay-{2}-more cast mode not modeled
    )
}
