//! Concealing Curtains // Revealing Eye — `{B}` Wall creature 0/4 with Defender.
//! Front face: {2}{B}: Transform this creature. Activate only as a sorcery.
//! Back face (Revealing Eye): Menace. When this creature transforms into
//! Revealing Eye, target opponent reveals their hand. You may choose a nonland
//! card from it. If you do, that player discards that card, then draws a card.
//!
//! GAP: "When this creature transforms into Revealing Eye" — back-face-only
//! transform-into trigger not modeled (triggered abilities live on the
//! CardDefinition, not the face; no face-gate on TriggeredAbilityDef).
//! The back face has Menace as a keyword in its characteristics.
//! GAP: "choose a nonland card from opponent's hand" selection is not
//! expressible; the triggered effect (discard then draw) is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Concealing Curtains");
    let wall_sub = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Revealing Eye");
    let eye_sub = reg.interner_mut().intern("Eye");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(eye_sub);
    back_subtypes.0.insert(horror_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Menace],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: back-face-only "transforms into Revealing Eye" trigger not modeled.

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {2}{B}: Transform this creature. Activate only as a sorcery.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: transform_self,
            }),
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
