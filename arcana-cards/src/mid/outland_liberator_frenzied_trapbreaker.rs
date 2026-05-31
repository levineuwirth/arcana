//! Outland Liberator // Frenzied Trapbreaker (transforming DFC, layout "transform").
//! Front (Outland Liberator — {1}{G} Creature — Human Werewolf, 2/2):
//!   {1}, Sacrifice this creature: Destroy target artifact or enchantment.
//!   Daybound.
//! Back (Frenzied Trapbreaker — Creature — Werewolf, 2/2 — printed stats):
//!   {1}, Sacrifice this creature: Destroy target artifact or enchantment.
//!   Whenever this creature attacks, destroy target artifact or enchantment
//!     defending player controls.
//!   Nightbound.
//!
//! GAPs:
//! - Daybound / Nightbound: no KeywordAbility variant exists for these
//!   day/night-coupled keywords, and the automatic day/night transform is not
//!   wired for this card; the keywords are not modeled.
//! - Back-face attack trigger: "defending player controls" cannot be tied to
//!   the actual defending player from the filter — approximated as an
//!   opponent-controlled artifact or enchantment.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition, CardFace,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Outland Liberator");
    let human = reg.interner_mut().intern("Human");
    let werewolf = reg.interner_mut().intern("Werewolf");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Frenzied Trapbreaker");
    let back_werewolf = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Both faces: {1}, Sacrifice this creature: Destroy target
            // artifact or enchantment. (Shared — no face gate.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice this creature: Destroy target artifact or enchantment."
                    .to_string(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::ENCHANTMENT,
                        )),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_target_artifact_or_enchantment,
            })
            // Back-only (face 1): whenever this creature attacks, destroy target
            // artifact or enchantment defending player controls.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_destroy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(
                                TypeLine::ARTIFACT | TypeLine::ENCHANTMENT,
                            ))
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_trigger_face_gate(1, 1),
    )
}

fn destroy_target_artifact_or_enchantment(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

fn attack_destroy(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}
