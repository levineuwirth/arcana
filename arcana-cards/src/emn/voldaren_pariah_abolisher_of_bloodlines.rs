//! Voldaren Pariah // Abolisher of Bloodlines
//!
//! Front: {3}{B}{B} Creature — Vampire Horror 3/3.
//! Flying.
//! Sacrifice three other creatures: Transform this creature. (Activated ability;
//!   wired via ActivationCost::sacrifice_other = creature, sacrifice_other_count = 3.
//!   The enumerator excludes the source, matching "three OTHER creatures".)
//! GAP: Madness {B}{B}{B} — madness mechanic not modeled.
//!
//! Back: Creature — Eldrazi Vampire. Flying.
//! When this creature transforms into Abolisher of Bloodlines, target opponent
//!   sacrifices three creatures of their choice. (Back-face triggered ability,
//!   gated to face 1; target opponent via TargetFilter::Player + Opponent, then
//!   Effect::Sacrifice posts the three-creature choice to that player.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voldaren Pariah");
    let sub_vampire = reg.interner_mut().intern("Vampire");
    let sub_horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_vampire);
    subtypes.0.insert(sub_horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Abolisher of Bloodlines");
    let sub_eldrazi = reg.interner_mut().intern("Eldrazi");
    let sub_vampire_b = reg.interner_mut().intern("Vampire");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(sub_eldrazi);
    back_subtypes.0.insert(sub_vampire_b);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Madness {B}{B}{B} not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: "Sacrifice three other creatures: Transform this creature."
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice three other creatures: Transform this creature.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(ObjectFilter::creature()),
                    sacrifice_other_count: 3,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            })
            // Back face: "When this creature transforms into Abolisher of
            // Bloodlines, target opponent sacrifices three creatures."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: on_transform_opponent_sacrifices_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            })
            .with_trigger_face_gate(1, 1),
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn on_transform_opponent_sacrifices_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Sacrifice {
        player: *p,
        filter: ObjectFilter::creature(),
        count: 3,
    }]
}
