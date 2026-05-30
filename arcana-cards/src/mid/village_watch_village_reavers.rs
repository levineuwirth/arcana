//! Village Watch // Village Reavers — `{4}{R}` Human Werewolf creature 4/3.
//! Front: Haste. Daybound (day/night cycle keyword — GAP: not modeled).
//! Back: Wolves and Werewolves you control have haste. Nightbound (GAP: not modeled).
//!
//! GAP: Daybound/Nightbound and the day/night cycle are not modeled in the engine.
//! GAP: Back face "Wolves and Werewolves you control have haste" — static continuous effect
//! granting keywords to subtypes is not modeled. Not authored.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Village Watch");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Haste],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Village Reavers");
    let mut back_subtypes = SubtypeSet::default();
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    back_subtypes.0.insert(back_werewolf_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            // GAP: "Wolves and Werewolves you control have haste" — continuous static ability
            // not modeled. No keywords placed here.
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // GAP: Daybound — precise "no spells cast on your last turn" transform condition
            // not modeled.
            // GAP: back-face-only triggered ability not modeled.
    )
}
