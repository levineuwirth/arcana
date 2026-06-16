//! Spike Breeder — `{3}{G}` 0/0 Spike. Enters with three +1/+1
//! counters.
//! "{2}, Remove a +1/+1 counter from this creature: Put a +1/+1
//! counter on target creature."
//! "{2}, Remove a +1/+1 counter from this creature: Create a 1/1 green
//! Spike creature token."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spike Breeder");
    let spike = reg.interner_mut().intern("Spike");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spike);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    // GAP: "enters with three +1/+1 counters" — no Characteristics field /
    // ETB-counter replacement is shown in the API; the entry-counter rider
    // is not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Remove a +1/+1 counter from this creature: Put a +1/+1 counter on target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_counter_on_target,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Remove a +1/+1 counter from this creature: Create a 1/1 green Spike creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_spike_token,
            }),
    )
}

fn put_counter_on_target(
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
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn make_spike_token(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let spike = reg.interner().lookup("Spike").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spike);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: spike,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
