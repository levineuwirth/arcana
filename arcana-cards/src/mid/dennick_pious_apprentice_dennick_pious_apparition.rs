//! Dennick, Pious Apprentice // Dennick, Pious Apparition
//!
//! FRONT: {W}{U} — Legendary Creature — Human Soldier (2/3)
//! Lifelink.
//! Cards in graveyards can't be the targets of spells or abilities. (GAP: static graveyard
//! protection effect not modeled.)
//! Disturb {2}{W}{U} — You may cast this from your graveyard transformed for its disturb cost.
//! (GAP: Disturb keyword not in the engine keyword surface.)
//!
//! BACK: Dennick, Pious Apparition — Legendary Creature — Spirit Soldier (3/4)
//! Flying.
//! Whenever one or more creature cards are put into graveyards from anywhere, investigate.
//! This ability triggers only once each turn.
//! (GAP: back-face-only triggered ability not auto-installed on transform.)
//! If Dennick would be put into a graveyard from anywhere, exile it instead.
//! (GAP: "exile instead of graveyard" replacement not modeled.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dennick, Pious Apprentice");
    let human_sub = reg.interner_mut().intern("Human");
    let soldier_sub = reg.interner_mut().intern("Soldier");

    let mut front_subtypes = SubtypeSet::new();
    front_subtypes.insert(human_sub);
    front_subtypes.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Dennick, Pious Apparition");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let soldier_sub2 = reg.interner_mut().intern("Soldier");

    let mut back_subtypes = SubtypeSet::new();
    back_subtypes.insert(spirit_sub);
    back_subtypes.insert(soldier_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Back-face triggers (investigate on creature death, exile replacement) not modeled —
    // back-face-only triggered abilities are not auto-installed on transform.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
    )
}
