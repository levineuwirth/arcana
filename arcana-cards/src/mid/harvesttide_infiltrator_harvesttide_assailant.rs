//! Harvesttide Infiltrator // Harvesttide Assailant
//!
//! Front face: `{2}{R}` Creature — Human Werewolf 3/2 with Trample.
//! Daybound (If a player casts no spells during their own turn, it becomes night next turn.)
//!
//! Back face: Creature — Werewolf 3/2 with Trample.
//! Nightbound (If a player casts at least two spells during their own turn, it becomes day next turn.)
//!
//! GAP: Daybound/Nightbound keywords not modeled — day/night cycle not implemented.
//! GAP: The precise werewolf transform conditions ("no spells cast last turn" / "two or more
//! spells cast this turn") are not expressible with the available TriggerCondition variants.
//! Transform is not wired (no trigger can fire it faithfully).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harvesttide Infiltrator");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human_sub);
    front_subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        // GAP: Daybound keyword not modeled.
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Harvesttide Assailant");
    // Reuse the already-interned "Werewolf" symbol for the back-face subtype.
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Trample],
            // GAP: Nightbound keyword not modeled.
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Daybound/Nightbound transform triggers not wired — day/night cycle not implemented.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
