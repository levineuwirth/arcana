//! Kookus — `{3}{R}{R}` 3/5 Djinn with Trample.
//! "At the beginning of your upkeep, if you don't control a creature named
//!  Keeper of Kookus, this creature deals 3 damage to you and attacks this
//!  turn if able.
//!  {R}: This creature gets +1/+0 until end of turn."

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kookus");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    // intern the named-creature reference for the intervening-if filter
    let _keeper = reg.interner_mut().intern("Keeper of Kookus");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_no_keeper),
                effect: upkeep_damage_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}: This creature gets +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
    )
}

fn if_no_keeper(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    let nm = reg.interner().lookup("Keeper of Kookus");
    let filter = ObjectFilter {
        name: nm,
        types: Some(TypeLine::CREATURE.into()),
        ..ObjectFilter::default()
    };
    conditions::you_control_at_most(s, you, &filter, 0)
}

fn upkeep_damage_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "and attacks this turn if able" — a must-attack-this-turn rider is
    // not expressible (no self-targeted forced-attack effect).
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(trig.controller),
        amount: 3,
    }]
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
