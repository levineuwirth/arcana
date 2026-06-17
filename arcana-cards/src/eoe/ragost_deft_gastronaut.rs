//! Ragost, Deft Gastronaut — `{R}{W}` 2/2 Legendary Lobster Citizen.
//! "Artifacts you control are Foods … and have '{2},{T},Sacrifice this artifact:
//!  You gain 3 life.'" (ability-granting static — GAP)
//! "{1},{T},Sacrifice a Food: Ragost deals 3 damage to each opponent."
//! "At the beginning of each end step, if you gained life this turn, untap Ragost."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ragost, Deft Gastronaut");
    let lobster = reg.interner_mut().intern("Lobster");
    let citizen = reg.interner_mut().intern("Citizen");
    let food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lobster);
    subtypes.0.insert(citizen);

    let sac_food = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .with_subtype_sym(food);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Artifacts you control are Foods in addition to their other types and
    // have '{2},{T},Sacrifice this artifact: You gain 3 life.'" — an ability- and
    // type-granting static is not expressible with the demonstrated API.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice a Food: Ragost deals 3 damage to each opponent.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(sac_food),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: damage_each_opponent,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_gained_life),
                effect: untap_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_gained_life(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::you_gained_life_this_turn(s, you)
}

fn damage_each_opponent(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opps = script::opponents(state, ctx.controller);
    opps.into_iter()
        .map(|p| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 3,
        })
        .collect()
}

fn untap_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Untap { target: trig.source }]
}
