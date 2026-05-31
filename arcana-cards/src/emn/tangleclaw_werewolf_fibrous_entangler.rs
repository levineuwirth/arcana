//! Tangleclaw Werewolf // Fibrous Entangler (transforming DFC, layout "transform")
//!
//! Front face: Tangleclaw Werewolf — {2}{G}{G} Creature — Werewolf Horror, 2/4 (G).
//!   Vigilance.
//!   This creature can block an additional creature each combat.
//!   {6}{G}: Transform this creature.
//! Back face: Fibrous Entangler — Creature — Eldrazi Werewolf, 2/4 (G).
//!   Vigilance.
//!   This creature must be blocked if able.
//!   This creature can block an additional creature each combat.
//!
//! GAP: "can block an additional creature each combat" (front and back) and
//!   "must be blocked if able" (back) are static combat-rule modifiers with no
//!   demonstrated Effect/static-ability surface — not expressible. The directional
//!   activated transform ability and the printed keywords/stats ARE authored.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tangleclaw Werewolf");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let horror = reg.interner_mut().intern("Horror");
    let eldrazi = reg.interner_mut().intern("Eldrazi");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(werewolf);
    front_subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Fibrous Entangler");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(eldrazi);
    back_subtypes.0.insert(werewolf);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Vigilance],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: {6}{G}: Transform this creature. Front-only.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}{G}: Transform this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}{G}").expect("valid cost"),
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: transform_self,
            }),
    )
}

fn transform_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
