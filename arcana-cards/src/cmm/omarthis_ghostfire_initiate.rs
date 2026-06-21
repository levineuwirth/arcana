//! Omarthis, Ghostfire Initiate — `{X}{X}` 0/0 Legendary Spirit Snake with
//! Manifest.
//!
//! Oracle:
//! * Omarthis enters with X +1/+1 counters on it.
//! * Whenever you put one or more +1/+1 counters on another colorless creature,
//!   you may put a +1/+1 counter on Omarthis.
//! * When Omarthis dies, manifest a number of cards from the top of your
//!   library equal to the number of counters on it.
//!
//! Manifest is listed by Scryfall but is the spell-side keyword reminder; the
//! card has no usable `KeywordAbility::Manifest` variant, so the keyword line
//! is empty and Manifest appears only via `Effect::Manifest` in the death
//! trigger. "Enters with X +1/+1 counters" is an as-enters replacement with no
//! expressible hook — GAP'd. The colorless-creature counter trigger uses
//! `CounterAdded` with `TriggerSelf::AnotherMatching` (per the loop-safety
//! invariant, since the effect places a counter; AnotherMatching also excludes
//! Omarthis itself so its self-counter never re-triggers). The "may" is
//! approximated as always doing it. The death trigger manifests one card per
//! +1/+1 counter on the dying object.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Omarthis, Ghostfire Initiate");
    let spirit = reg.interner_mut().intern("Spirit");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{X}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // "colorless creature" = a creature with none of the five colors.
    let colorless_creature = ObjectFilter::creature().without_colors(
        ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
    );

    // GAP: "Omarthis enters with X +1/+1 counters on it." — as-enters counter
    // placement has no expressible hook in this card class.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::AnotherMatching(colorless_creature),
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: counter_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: death_manifest,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counter_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "you may put a +1/+1 counter on Omarthis" — the optional is approximated
    // as always doing it.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn death_manifest(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    let n = state
        .objects
        .get(id)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    (0..n)
        .map(|_| Effect::Manifest { player: trig.controller })
        .collect()
}
