//! Thraben Gargoyle // Stonewing Antagonizer (transforming DFC, layout "transform").
//! Front (Thraben Gargoyle — {1} Artifact Creature — Gargoyle, 2/2):
//!   Defender.
//!   {6}: Transform this creature.
//! Back (Stonewing Antagonizer — Artifact Creature — Gargoyle Horror, 2/2):
//!   Flying.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition, CardFace,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thraben Gargoyle");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(gargoyle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Stonewing Antagonizer");
    let back_gargoyle = reg.interner_mut().intern("Gargoyle");
    let horror = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_gargoyle);
    back_subtypes.0.insert(horror);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front (face 0): {6}: Transform this creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}: Transform this creature.".to_string(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            }),
    )
}

fn transform_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
