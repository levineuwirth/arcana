//! Fearful Villager // Fearsome Werewolf
//!
//! Front: Creature — Human Werewolf {2}{R}, 2/3
//!   Menace
//!   Daybound (If a player casts no spells during their own turn, it becomes night next turn.)
//!
//! Back: Creature — Werewolf (Fearsome Werewolf)
//!   Menace
//!   Nightbound (If a player casts at least two spells during their own turn, it becomes day next turn.)
//!
//! GAP: Daybound / Nightbound day/night cycle mechanics not modeled in the engine.
//!      Transform is registered but the precise daybound/nightbound trigger conditions
//!      (no spells cast / 2+ spells cast this turn) are not expressible as TriggerCondition variants.
//!      The card's front/back faces and Menace keyword are fully registered.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fearful Villager");
    let back_name = reg.interner_mut().intern("Fearsome Werewolf");

    let mut front_subtypes = SubtypeSet::default();
    let human = reg.interner_mut().intern("Human");
    let werewolf = reg.interner_mut().intern("Werewolf");
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(werewolf);

    let mut back_subtypes = SubtypeSet::default();
    let werewolf_back = reg.interner_mut().intern("Werewolf");
    back_subtypes.0.insert(werewolf_back);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        keywords: vec![KeywordAbility::Menace],
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Menace],
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Daybound/Nightbound day-night cycle transform triggers not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
    )
}
