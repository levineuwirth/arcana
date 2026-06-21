//! Dina, Soul Steeper — `{B}{G}` 1/3 Legendary Dryad Druid.
//!
//! Oracle:
//! * "Whenever you gain life, each opponent loses 1 life." — LifeGained (You)
//!   trigger; one LoseLife(1) per opponent. Fully expressible.
//! * "{1}, Sacrifice another creature: Dina gets +X/+0 until end of turn, where
//!   X is the sacrificed creature's power." — the cost ({1} + sacrifice another
//!   creature) is expressible, but X (the sacrificed creature's power) is not
//!   available at resolution (the sacrificed permanent is gone and its power
//!   isn't exposed via ActivationContext). The activated ability is registered;
//!   the pump amount is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dina, Soul Steeper");
    let dryad = reg.interner_mut().intern("Dryad");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: opponents_lose_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice another creature: Dina, Soul Steeper gets \
                       +X/+0 until end of turn, where X is the sacrificed \
                       creature's power.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").unwrap(),
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_by_sacrificed_power,
            }),
    )
}

fn opponents_lose_one(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::LoseLife { player: opp, amount: 1 })
        .collect();
    if effects.is_empty() {
        Vec::new()
    } else {
        vec![Effect::Sequence(effects)]
    }
}

fn pump_by_sacrificed_power(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+X/+0 where X is the sacrificed creature's power" — the sacrificed
    // permanent (paid as the cost) is gone by resolution and its power isn't
    // exposed via ActivationContext, so the dynamic amount cannot be computed.
    Vec::new()
}
