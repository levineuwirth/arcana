//! Chaos Spewer — `{2}{B/R}` 5/4 black/red Goblin Warlock.
//! "When this creature enters, you may pay {2}. If you don't, blight 2.
//! (To blight 2, put two -1/-1 counters on a creature you control.)"
//! GAP: 'blight' mechanic (put two -1/-1 counters on a creature you control)
//! — no Effect::Blight; CounterKind::MinusOneMinusOne + AddCounters could
//! work for the counters but 'choose a creature you control' targeting is
//! missing from the OptionalPayment else_effect. Emitting partial approximation.

use arcana_core::actions::OptionalPaymentKind;
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
    let name = reg.interner_mut().intern("Chaos Spewer");
    let goblin = reg.interner_mut().intern("Goblin");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_blight_or_pay,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_blight_or_pay(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'blight 2' else_effect — no player-chosen creature target in
    // OptionalPayment else_effect; using trig.source as placeholder target.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::MinusOneMinusOne,
            count: 2,
        })),
    }]
}
