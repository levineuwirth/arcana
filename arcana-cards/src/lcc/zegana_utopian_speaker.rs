//! Zegana, Utopian Speaker — `{2}{G}{U}` 4/4 Legendary Merfolk Wizard.
//!
//! * When Zegana enters, if you control another creature with a +1/+1 counter on
//!   it, draw a card.
//! * {4}{G}{U}: Adapt 4. (If this creature has no +1/+1 counters on it, put four
//!   +1/+1 counters on it.)
//! * Each creature you control with a +1/+1 counter on it has trample. (static — GAP)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zegana, Utopian Speaker");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: static "Each creature you control with a +1/+1 counter on it has trample"
        // — counter-conditioned continuous keyword grant has no static primitive here.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: intervening-if "if you control another creature with a +1/+1
                // counter on it" — no conditions:: helper for a counter-bearing-creature
                // board predicate; fires unconditionally.
                intervening_if: None,
                effect: etb_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{G}{U}: Adapt 4.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{G}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: adapt_4,
            }),
    )
}

fn etb_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}

/// Adapt 4: if this creature has no +1/+1 counters on it, put four +1/+1 counters on it.
fn adapt_4(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let existing = state
        .objects
        .get(ctx.source)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    if existing == 0 {
        vec![Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 4,
        }]
    } else {
        Vec::new()
    }
}
