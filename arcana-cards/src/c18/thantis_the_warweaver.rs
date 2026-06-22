//! Thantis, the Warweaver — `{3}{B}{R}{G}` Legendary 5/5 Creature — Spider.
//! "Vigilance, reach"
//! "All creatures attack each combat if able."
//! "Whenever a creature attacks you or a planeswalker you control, put a +1/+1
//!  counter on Thantis."
//!
//! Decomposition:
//! - Keyword line: Vigilance, Reach.
//! - GAP: "All creatures attack each combat if able." is a static combat-
//!   restriction continuous ability — no expressible Effect.
//! - "Whenever a creature attacks you or a planeswalker you control, …" → a
//!   CreatureAttacks triggered ability putting a +1/+1 counter on Thantis.
//!   GAP (approximation): the "attacks YOU or a planeswalker you control"
//!   defending-player restriction is not expressible via the attacker filter,
//!   so the trigger watches any creature attacking.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thantis, the Warweaver");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature(),
            },
            intervening_if: None,
            effect: counter_on_self,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn counter_on_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
