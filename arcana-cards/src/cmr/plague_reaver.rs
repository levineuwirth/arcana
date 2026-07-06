//! Plague Reaver — `{2}{B}` 6/5 Beast.
//! "At the beginning of your end step, sacrifice each other creature you
//! control."
//! "Discard two cards, Sacrifice this creature: Choose target opponent.
//! Return this creature to the battlefield under that player's control at
//! the beginning of their next upkeep."
//!
//! The end-step sacrifice is wired as a ForEach over the other creatures
//! you control. The activated ability's cost (discard two, sacrifice
//! self) is wired, but its delayed return-under-opponent's-control body
//! is not expressible (no delayed return-to-battlefield-under-another's-
//! control action), so the effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Plague Reaver");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_sacrifice_others,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard two cards, Sacrifice this creature: Choose target opponent. \
                       Return this creature to the battlefield under that player's control at \
                       the beginning of their next upkeep."
                    .into(),
                cost: ActivationCost {
                    sacrifice: true,
                    discard_other: Some(ObjectFilter::default()),
                    discard_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: return_under_opponent,
            }),
    )
}

fn end_step_sacrifice_others(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids: Vec<_> = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    )
    .into_iter()
    .filter(|id| *id != trig.source)
    .collect();
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: NULL_OBJECT_ID,
        }),
    }]
}

fn return_under_opponent(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Return this creature to the battlefield under that player's
    // control at the beginning of their next upkeep" — no delayed
    // return-from-graveyard-under-another-player's-control action.
    Vec::new()
}
