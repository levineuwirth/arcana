//! Rottenmouth Viper — `{5}{B}` 6/6 Elemental Snake.
//! "Whenever this creature enters or attacks, put a blight counter on it.
//! Then for each blight counter on it, each opponent loses 4 life unless
//! that player sacrifices a nonland permanent of their choice or discards
//! a card."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rottenmouth Viper");
    let elemental = reg.interner_mut().intern("Elemental");
    let snake = reg.interner_mut().intern("Snake");
    // Interned at register so the resolver can recover the counter kind.
    let _blight = reg.interner_mut().intern("blight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(snake);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: add_blight_then_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: add_blight_then_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_blight_then_drain(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(blight) = reg.interner().lookup("blight").map(CounterKind::Named) else {
        return Vec::new();
    };
    // GAP: "Then for each blight counter on it, each opponent loses 4 life
    // unless that player sacrifices a nonland permanent or discards a card."
    // The lose-life-unless-sacrifice-or-discard choice is not expressible
    // (OptionalPaymentKind has only Mana/Life). Only the counter is added.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: blight,
        count: 1,
    }]
}
