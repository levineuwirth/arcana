//! Crazed Firecat — `{5}{R}{R}` 4/4 red Elemental Cat. "When this creature
//! enters, flip a coin until you lose a flip. Put a +1/+1 counter on this
//! creature for each flip you won."
//!
//! Modeled as a recursive `Effect::FlipCoin` chain. Each win adds a counter
//! and schedules another flip; the lose branch terminates the loop.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// ObjectId is used in flip_once; import via objects module.
use arcana_core::objects::ObjectId;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crazed Firecat");
    let elemental = reg.interner_mut().intern("Elemental");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(cat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_flip_coins,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_flip_coins(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![flip_once(trig.source, trig.controller)]
}

fn flip_once(source: ObjectId, controller: PlayerId) -> Effect {
    Effect::FlipCoin {
        player: controller,
        win: Box::new(Effect::Sequence(vec![
            Effect::AddCounters {
                target: source,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            },
            flip_once(source, controller),
        ])),
        lose: None,
    }
}
