//! Vivi Ornitier — `{1}{U}{R}` 0/3 Legendary Wizard (U/R).
//! {0}: Add X mana in any combination of {U} and/or {R}, where X is
//!   Vivi's power. Activate only during your turn and only once each turn.
//! Whenever you cast a noncreature spell, put a +1/+1 counter on Vivi
//!   and it deals 1 damage to each opponent.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::conditions;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PlayerId, PtValue, SubtypeSet,
    SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vivi Ornitier");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // {0}: Add X mana in any combination of {U}/{R}, X = Vivi's power.
            // Only during your turn, only once each turn.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{0}: Add X mana in any combination of {U} and/or {R}, where X is Vivi Ornitier's power. Activate only during your turn and only once each turn.".into(),
                cost: ActivationCost {
                    activation_condition: Some(only_your_turn),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_x_mana,
            })
            // Whenever you cast a noncreature spell, put a +1/+1 counter on
            // Vivi and it deals 1 damage to each opponent.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: noncreature_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn only_your_turn(state: &GameState, source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::is_your_turn(state, source, you)
}

fn add_x_mana(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let x = script::power_of(state, ctx.source).max(0) as usize;
    if x == 0 {
        return Vec::new();
    }
    // GAP: the U/R combination is the controller's choice; producing red.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source); x],
    }]
}

fn noncreature_cast(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(opp),
            amount: 1,
        });
    }
    effects
}
