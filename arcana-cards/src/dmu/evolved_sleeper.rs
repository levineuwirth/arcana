//! Evolved Sleeper — `{B}` 1/1 black Human.
//! A three-step level-up creature, all activated:
//!   {B}: becomes a Human Cleric with base P/T 2/2.
//!   {1}{B}: if it's a Cleric, put a deathtouch counter on it and it becomes a
//!     Phyrexian Human Cleric with base P/T 3/3.
//!   {1}{B}{B}: if it's a Phyrexian, put a +1/+1 counter on it, then draw a card
//!     and lose 1 life.
//! Subtype changes and the "if Cleric/Phyrexian" preconditions are not
//! expressible; the base-P/T set and counter/draw/life payloads are.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Evolved Sleeper");
    let human = reg.interner_mut().intern("Human");
    let _cleric = reg.interner_mut().intern("Cleric");
    let _phyrexian = reg.interner_mut().intern("Phyrexian");
    let _deathtouch = reg.interner_mut().intern("deathtouch");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}: This creature becomes a Human Cleric with base power and toughness 2/2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_cleric,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}: If this creature is a Cleric, put a deathtouch counter on it and it becomes a Phyrexian Human Cleric with base power and toughness 3/3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_phyrexian,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}{B}: If this creature is a Phyrexian, put a +1/+1 counter on it, then you draw a card and you lose 1 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grow_and_draw,
            }),
    )
}

fn become_cleric(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: gaining the Cleric subtype is not expressible (AddType is TypeLine-only).
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 2,
        toughness: 2,
        duration: Duration::Permanent,
    }]
}

fn become_phyrexian(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "if it's a Cleric" precondition and the Phyrexian subtype gain
    // are not expressible; the deathtouch counter and base-P/T set are.
    let dt = reg
        .interner()
        .lookup("deathtouch")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::PlusOnePlusOne);
    vec![
        Effect::AddCounters { target: ctx.source, kind: dt, count: 1 },
        Effect::SetBasePT {
            target: ctx.source,
            power: 3,
            toughness: 3,
            duration: Duration::Permanent,
        },
    ]
}

fn grow_and_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "if it's a Phyrexian" precondition is not expressible.
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::LoseLife { player: ctx.controller, amount: 1 },
    ]
}
