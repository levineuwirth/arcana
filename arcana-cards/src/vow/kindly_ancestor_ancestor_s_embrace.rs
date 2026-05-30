//! Kindly Ancestor // Ancestor's Embrace
//!
//! FRONT: {2}{W} — Creature — Spirit (2/3)
//! Lifelink.
//! Disturb {1}{W} — You may cast this card from your graveyard transformed for its disturb cost.
//! (GAP: Disturb keyword not in the engine keyword surface — not modeled.)
//!
//! BACK: Ancestor's Embrace — Enchantment — Aura
//! Enchant creature. Enchanted creature has lifelink.
//! If Ancestor's Embrace would be put into a graveyard from anywhere, exile it instead.
//! (GAP: back-face is an Aura — "enchant creature" attach behavior on transform not modeled.)
//! (GAP: "exile instead of graveyard" replacement effect not modeled.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kindly Ancestor");
    let spirit_sub = reg.interner_mut().intern("Spirit");

    let mut front_subtypes = SubtypeSet::new();
    front_subtypes.insert(spirit_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ancestor's Embrace");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::new();
    back_subtypes.insert(aura_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            // GAP: Aura "enchant creature" + grant lifelink to enchanted creature not modeled.
            // GAP: "exile instead of graveyard" replacement not modeled.
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
    )
}
