//! Ostrich-Horse — `{2}{G}` 3/1 green Bird Horse. "When this creature enters, mill
//! three cards. You may put a land card from among them into your hand. If you don't,
//! put a +1/+1 counter on this creature." Keywords: Mill.
//! GAP: "choose among milled cards" conditional not in engine effect catalog;
//! emitting mill 3 and counter on self (best-effort).

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
    let name = reg.interner_mut().intern("Ostrich-Horse");
    let bird = reg.interner_mut().intern("Bird");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(horse);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_mill(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose a land card from milled cards or counter on self" branching not supported
    vec![
        Effect::Mill { player: trig.controller, count: 3 },
        Effect::AddCounters { target: trig.source, kind: CounterKind::PlusOnePlusOne, count: 1 },
    ]
}
