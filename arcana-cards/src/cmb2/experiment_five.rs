//! Experiment Five — `{G}` 1/1 Bear Snake Mutant.
//! `{1}{Z}: Put a +1/+2 counter on Experiment Five.`
//! GAP: "{Z}" is a custom mana symbol (paid with mana from source producing 2+ colors) —
//! not parseable by ManaCost::parse; using {1} as approximation.
//! GAP: "+1/+2 counter" — only +1/+1 and -1/-1 counter kinds available; using PlusOnePlusOne.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Experiment Five");
    let bear = reg.interner_mut().intern("Bear");
    let snake = reg.interner_mut().intern("Snake");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    subtypes.0.insert(snake);
    subtypes.0.insert(mutant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{Z}: Put a +1/+2 counter on Experiment Five. ({Z} is paid with mana from a 2+ color source.)".into(),
                cost: ActivationCost {
                    // GAP: {Z} not parseable; using {2} as approximation
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_counter,
            }),
    )
}

fn add_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+1/+2 counter" not in CounterKind; using PlusOnePlusOne
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
