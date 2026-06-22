//! Knighted Myr — `{2}{W}` 2/2 Artifact Creature — Myr Knight.
//! `{2}{W}: Adapt 1.` (If this creature has no +1/+1 counters on it, put a
//! +1/+1 counter on it.)
//! "Whenever one or more +1/+1 counters are put on this creature, it gains
//! double strike until end of turn."
//!
//! Decomposition:
//! - Keyword line: Adapt — NOT in the usable keyword surface, so `keywords:
//!   vec![]`. (See GAP below.)
//! - Activated `{2}{W}: Adapt 1` → an ActivatedAbilityDef putting a +1/+1
//!   counter on this creature.
//! - "Whenever one or more +1/+1 counters are put on this creature, …" →
//!   a CounterAdded(TriggerSelf::Source) triggered ability granting double
//!   strike until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knighted Myr");
    let myr = reg.interner_mut().intern("Myr");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(myr);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: keyword `Adapt` not in usable KeywordAbility surface; the adapt
        // activation itself is modeled below as an activated ability.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}: Adapt 1.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: adapt_one,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: gain_double_strike,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Adapt 1: put a +1/+1 counter on this creature.
/// GAP: the "if this creature has no +1/+1 counters on it" guard (CR 702.139)
/// is not expressible with the documented Effect/Condition surface, so the
/// counter is added unconditionally.
fn adapt_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn gain_double_strike(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::DoubleStrike,
        duration: Duration::EndOfTurn,
    }]
}
