//! Lantern Bearer // Lanterns' Lift — `{U}` Spirit creature 1/1 with Flying.
//! Disturb {2}{U}: You may cast this card from your graveyard transformed for its
//! disturb cost.
//! Back face "Lanterns' Lift": Enchantment — Aura. Enchant creature.
//! Enchanted creature gets +1/+1 and has flying.
//! If Lanterns' Lift would be put into a graveyard from anywhere, exile it instead.
//!
//! GAP: Disturb keyword is not in the engine surface — emitted as no keyword.
//! GAP: Back-face Aura "+1/+1 and flying to enchanted creature" is a static effect
//! not expressible with the trigger/effect API.
//! GAP: Back-face "would be put into a graveyard, exile it instead" is a replacement
//! effect not modeled.
//! GAP: back-face-only triggered/static abilities not auto-installed on transform.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lantern Bearer");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Disturb {2}{U} not in the engine keyword surface
        ..Default::default()
    };

    // Back face "Lanterns' Lift": Enchantment — Aura
    let back_name = reg.interner_mut().intern("Lanterns' Lift");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            // GAP: "enchanted creature gets +1/+1 and has flying" static Aura effect not modeled
            // GAP: "if Lanterns' Lift would be put into a graveyard from anywhere, exile it instead" not modeled
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
