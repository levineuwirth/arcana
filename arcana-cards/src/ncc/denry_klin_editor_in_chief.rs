//! Denry Klin, Editor in Chief — `{2}{W}{U}` 2/2 Legendary Cat Advisor.
//!
//! Oracle:
//! * "Denry Klin enters with your choice of a +1/+1, first strike, or
//!   vigilance counter on it." — modeled as an ETB trigger that adds a
//!   +1/+1 counter (deterministic pick of one valid mode). PARTIAL GAP: the
//!   player's choice among +1/+1 / first-strike / vigilance counters is not
//!   expressible (no counter-kind choice; first-strike/vigilance keyword
//!   counters have no `CounterKind` variant).
//! * "Whenever a nontoken creature you control enters, if Denry Klin has
//!   counters on it, put the same number of each kind of counter on that
//!   creature." — GAP: requires enumerating Denry's arbitrary counter kinds
//!   and mirroring each onto the entering creature; not expressible with
//!   the demonstrated primitives (intervening-if on "has counters" is also
//!   only available for known kinds).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Denry Klin, Editor in Chief");
    let cat = reg.interner_mut().intern("Cat");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enters_with_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn enters_with_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // PARTIAL: deterministic +1/+1 counter (one of the three valid modes).
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
