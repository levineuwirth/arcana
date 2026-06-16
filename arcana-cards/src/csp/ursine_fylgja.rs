//! Ursine Fylgja — `{4}{W}` 3/3 white Spirit Bear.
//!
//! Oracle text:
//! * "This creature enters with four healing counters on it." —
//!   modeled as a SelfEntersBattlefield trigger placing 4 healing
//!   counters on itself.
//! * "Remove a healing counter from this creature: Prevent the next 1
//!   damage that would be dealt to this creature this turn." —
//!   activated; cost removes one healing counter; prevents 1 damage to
//!   this creature until end of turn.
//! * "{2}{W}: Put a healing counter on this creature." — activated
//!   mana ability text but NOT a mana ability (it adds a counter).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ursine Fylgja");
    let spirit = reg.interner_mut().intern("Spirit");
    let bear = reg.interner_mut().intern("Bear");
    let healing = reg.interner_mut().intern("healing");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(bear);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_four_healing_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove a healing counter from this creature: Prevent \
                       the next 1 damage that would be dealt to this creature \
                       this turn."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Named(healing), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_next_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}: Put a healing counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_healing_counter,
            }),
    )
}

fn etb_four_healing_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(healing) = reg.interner().lookup("healing") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(healing),
        count: 4,
    }]
}

fn prevent_next_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(ctx.source),
        amount: Some(1),
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn add_healing_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(healing) = reg.interner().lookup("healing") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Named(healing),
        count: 1,
    }]
}
