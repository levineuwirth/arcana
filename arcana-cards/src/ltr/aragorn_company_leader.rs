//! Aragorn, Company Leader — `{1}{G}{W}` 3/3 Legendary Human Ranger with
//! Vigilance and Deathtouch.
//!
//! * "Whenever the Ring tempts you, if you chose a creature other than Aragorn
//!   as your Ring-bearer, put your choice of a counter from among first strike,
//!   vigilance, deathtouch, and lifelink on Aragorn." — There is no
//!   "Ring tempts you" TriggerCondition variant, and the keyword-counter choice
//!   (first strike / vigilance / deathtouch / lifelink) is not expressible.
//!   GAP'd entirely; we keep a no-op effect on the closest available trigger so
//!   the ability slot is recorded. The intervening-if (Ring-bearer choice) is
//!   also GAP'd.
//! * "Whenever you put one or more counters on Aragorn, put one of each of those
//!   kinds of counters on up to one other target creature." — wired to
//!   `CounterAdded` on the source. The "one of each of those kinds" payload
//!   (mirroring the kinds just added) is not expressible (no event accessor for
//!   the added-counter kinds, no keyword-counter Effect) → effect GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aragorn, Company Leader");
    let human = reg.interner_mut().intern("Human");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ranger);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Deathtouch],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "Whenever the Ring tempts you" has no
                // TriggerCondition variant; using the closest placeholder.
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if "if you chose a creature other than
                // Aragorn as your Ring-bearer" not expressible.
                intervening_if: None,
                effect: ring_tempts_choice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: None,
                    chapter: None,
                },
                intervening_if: None,
                effect: mirror_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn ring_tempts_choice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put your choice of a counter from among first strike, vigilance,
    // deathtouch, and lifelink on Aragorn" — keyword-counter choice not
    // expressible.
    Vec::new()
}

fn mirror_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put one of each of those kinds of counters on up to one other
    // target creature" — no accessor for the kinds just added and no
    // keyword-counter Effect to replicate them.
    Vec::new()
}
