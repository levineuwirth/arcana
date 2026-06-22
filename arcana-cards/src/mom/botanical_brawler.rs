//! Botanical Brawler — `{G}{W}` 0/0 Creature — Elemental Warrior. G/W.
//! Trample.
//! "This creature enters with two +1/+1 counters on it." —
//! EntersWithSpec::Counters.
//! "Whenever one or more +1/+1 counters are put on another permanent you
//! control, if it's the first time +1/+1 counters have been put on that
//! permanent this turn, put a +1/+1 counter on this creature." —
//! CounterAdded watching ANOTHER permanent you control (AnotherMatching, so
//! the self-counter it places doesn't re-trigger). GAP (fidelity): the
//! "first time this turn" intervening clause has no expressible predicate,
//! so it fires on every batch of +1/+1 counters.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Botanical Brawler");
    let elemental = reg.interner_mut().intern("Elemental");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::PlusOnePlusOne,
                count: 2,
            })
            // GAP (fidelity): "if it's the first time +1/+1 counters have been
            // put on that permanent this turn" — no expressible predicate.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::AnotherMatching(
                        ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
                    ),
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: counter_on_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counter_on_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
