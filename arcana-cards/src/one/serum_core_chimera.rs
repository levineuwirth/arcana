//! Serum-Core Chimera — `{2}{U}{R}` 2/4 Creature — Phyrexian Chimera.
//! Flying.
//! Whenever you cast a noncreature spell, put an oil counter on this creature.
//! Remove three oil counters from this creature: Draw a card. Then you may
//! discard a nonland card. When you discard a card this way, this creature
//! deals 3 damage to target creature or planeswalker. Activate only as a
//! sorcery.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serum-Core Chimera");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let chimera = reg.interner_mut().intern("Chimera");
    let oil = reg.interner_mut().intern("oil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(chimera);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: oil_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Remove three oil counters: Draw a card." — the draw is
            // expressible; "Then you may discard a nonland card. When you
            // discard a card this way, deal 3 damage to target creature or
            // planeswalker." is a reflexive optional-discard + targeted-damage
            // rider that can't be wired alongside the draw here.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove three oil counters from this creature: Draw a card. Then you may discard a nonland card. When you discard a card this way, this creature deals 3 damage to target creature or planeswalker. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Named(oil), 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_card_then,
            }),
    )
}

fn oil_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(oil) = reg.interner().lookup("oil").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: oil,
        count: 1,
    }]
}

fn draw_card_then(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Then you may discard a nonland card. When you discard a card this
    // way, this creature deals 3 damage to target creature or planeswalker."
    // — a reflexive optional discard with a conditional targeted-damage rider.
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
