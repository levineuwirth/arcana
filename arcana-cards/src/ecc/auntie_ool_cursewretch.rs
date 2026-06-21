//! Auntie Ool, Cursewretch — `{1}{B}{R}{G}` 4/4 Legendary Goblin Warlock.
//!
//! Ward—Blight 2. (A non-mana, "put two -1/-1 counters on a creature you
//! control" ward cost — NOT expressible as `KeywordAbility::Ward(ManaCost)`,
//! which only models a mana ward cost.)
//! Whenever one or more -1/-1 counters are put on a creature, draw a card if
//! you control that creature; otherwise its controller loses 1 life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Auntie Ool, Cursewretch");
    let goblin = reg.interner_mut().intern("Goblin");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: keyword — Ward—Blight 2 is a non-mana ward cost; only mana ward
        // costs are expressible as KeywordAbility::Ward(ManaCost).
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CounterAdded {
                on: TriggerSelf::AnyMatching(ObjectFilter::creature()),
                kind: Some(CounterKind::MinusOneMinusOne),
                chapter: None,
            },
            intervening_if: None,
            effect: on_minus_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "draw a card if you control that creature. If you don't control it, its
/// controller loses 1 life."
///
/// GAP: the engine exposes no typed accessor for the object that received the
/// counter (`CounterAdded` carries no entering/attacking/dying object), and
/// matching `trig.trigger_event` directly is forbidden. We cannot tell whether
/// the affected creature is one you control vs. an opponent's, so the
/// controller-conditional branch (draw vs. their-controller-loses-1-life) is
/// inexpressible. The trigger condition itself fires faithfully.
fn on_minus_counter(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}
