//! Soul Seizer // Ghastly Haunting — `{3}{U}{U}` Spirit creature 1/3
//! with Flying. When this creature deals combat damage to a player, you may
//! transform it. If you do, attach it to target creature that player controls.
//! Back face "Ghastly Haunting": Enchantment — Aura. Enchant creature.
//! You control enchanted creature.
//!
//! # GAP
//! The front-face triggered ability "when this deals combat damage to a
//! player, you may transform it; if you do, attach it to target creature
//! that player controls" requires:
//!   1. A combat-damage-to-player trigger condition (TriggerCondition::
//!      DamageDealt is not in the documented set).
//!   2. An optional transform + attach effect targeting a creature controlled
//!      by the damaged player.
//! The combat-damage trigger is not available — the trigger is GAP'd.
//! The back face is registered for completeness; "you control enchanted
//! creature" (ChangeControl as static Aura effect) is also a GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul Seizer");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: combat damage to player trigger not modeled — the transform +
    // attach-to-opponent's-creature effect cannot fire without it.

    let back_name = reg.interner_mut().intern("Ghastly Haunting");
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
            // GAP: "you control enchanted creature" static ability not modeled
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
