//! Delif's Cone — `{0}` artifact.
//! "{T}, Sacrifice this artifact: This turn, when target creature you
//! control attacks and isn't blocked, you may gain life equal to its
//! power. If you do, it assigns no combat damage this turn." Modeled
//! as an until-end-of-turn `GrantTriggeredAbility` (attacks-unblocked
//! trigger) on the chosen creature whose effect gains life equal to
//! that creature's power.
//!
//! GAP: "If you do, it assigns no combat damage this turn" — there is
//! no assigns-no-combat-damage effect; the life gain is wired without
//! the damage-forfeit rider (and the "you may" is resolved as always
//! taken).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Delif's Cone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Sacrifice this artifact: This turn, when target \
                       creature you control attacks and isn't blocked, you \
                       may gain life equal to its power. If you do, it \
                       assigns no combat damage this turn."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_unblocked_lifegain,
            },
        ),
    )
}

fn grant_unblocked_lifegain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::GrantTriggeredAbility {
        target: *id,
        ability: Box::new(TriggeredAbilityDef {
            id: GRANTED_TRIGGER_ID_BASE + 1,
            trigger_condition: TriggerCondition::SelfAttacksUnblocked,
            intervening_if: None,
            effect: gain_life_equal_power,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        duration: Duration::EndOfTurn,
    }]
}

fn gain_life_equal_power(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If you do, it assigns no combat damage this turn" — no
    // assigns-no-combat-damage effect; the optional life gain is taken
    // unconditionally.
    let amount = script::power_of(state, trig.source).max(0) as u32;
    vec![Effect::GainLife { player: trig.controller, amount }]
}
