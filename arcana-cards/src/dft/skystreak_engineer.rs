//! Skystreak Engineer — `{1}{U}` 1/3 Human Pilot with Flying.
//!
//! "Flying. Exhaust — {4}{U}: Put two +1/+1 counters on this creature.
//! (Activate each exhaust ability only once.)"
//!
//! Flying is a base keyword. The exhaust ability costs {4}{U} and puts
//! two +1/+1 counters on this creature. The Exhaust restriction
//! ("activate only once" — once per game) has no once-per-game cost
//! field in this API surface — GAP'd; the ability is otherwise faithful.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skystreak Engineer");
    let human = reg.interner_mut().intern("Human");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: Exhaust "activate only once" (once-per-game) restriction has
            // no once-per-game cost field; the ability is otherwise faithful.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {4}{U}: Put two +1/+1 counters on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_counters,
            }),
    )
}

fn add_two_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
