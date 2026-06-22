//! Reluctant Role Model — `{1}{W}` 2/2 Creature — Human Survivor.
//! Lifelink. (Survival is a keyword ability that gates the first trigger on
//! this creature being tapped; the keyword itself has no KeywordAbility variant
//! and is modeled as the trigger's intervening-if gate below — see GAP.)
//!
//! Decomposition:
//! 1. Keyword line: Lifelink. (Survival — GAP: no KeywordAbility::Survival.)
//! 2. "Survival — At the beginning of your second main phase, if this creature
//!    is tapped, put a flying, lifelink, or +1/+1 counter on it." Trigger fires
//!    at second (post-combat) main. GAP the body: the "if tapped" gate has no
//!    listed conditions:: predicate, and the player's choice of which counter
//!    (flying / lifelink / +1/+1) cannot be expressed in a triggered ability.
//! 3. "Whenever this creature or another creature you control dies, if it had
//!    counters on it, put those counters on up to one target creature."
//!    Dies trigger (any creature you control). GAP the body: there is no Effect
//!    that reads an arbitrary set of counters off the dying object and moves
//!    them, and "if it had counters" has no listed intervening-if predicate
//!    keyed on the dying object.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reluctant Role Model");
    let human = reg.interner_mut().intern("Human");
    let survivor = reg.interner_mut().intern("Survivor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(survivor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PostCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: survival_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: move_counters_on_death,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn survival_counter(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if this creature is tapped, put a flying, lifelink, or +1/+1
    // counter on it." No conditions:: predicate for source-is-tapped, and no
    // way to express the player's choice among three counter kinds in a
    // triggered ability. Effect omitted.
    Vec::new()
}

fn move_counters_on_death(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it had counters on it, put those counters on up to one target
    // creature." No Effect reads/moves an arbitrary multiset of counters off
    // the dying object. Effect omitted.
    Vec::new()
}
