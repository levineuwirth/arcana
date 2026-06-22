//! The Spike Cactus — `{1}{G}{G}` 0/0 Legendary Spike.
//!
//! Oracle:
//! The Spike Cactus enters with +1/+1 counters on it equal to the amount of
//! mana spent to cast it.
//! {2}, Remove a +1/+1 counter from The Spike Cactus: Put a +1/+1 counter on
//! target creature.
//!
//! Decomposition:
//! * "enters with +1/+1 counters equal to the mana spent to cast it" → the
//!   amount-of-mana-spent value is not exposed by any demonstrated `script::`
//!   helper, so this ETB rider is GAP'd (a fixed literal would be wrong).
//! * "{2}, Remove a +1/+1 counter: Put a +1/+1 counter on target creature" →
//!   a fully-wired activated ability (mana + remove-self-counter cost).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Spike Cactus");
    let spike = reg.interner_mut().intern("Spike");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spike);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "enters with +1/+1 counters equal to the amount of mana spent to
    // cast it" — mana-spent amount isn't exposed by any demonstrated helper.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, Remove a +1/+1 counter from The Spike Cactus: Put a +1/+1 counter on target creature.".into(),
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
            effect: add_counter_to_target,
        }),
    )
}

fn add_counter_to_target(
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
