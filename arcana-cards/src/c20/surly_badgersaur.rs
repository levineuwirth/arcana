//! Surly Badgersaur — `{3}{R}` 3/3 Creature — Badger Dinosaur.
//!
//! Whenever you discard a creature card, put a +1/+1 counter on this creature.
//! Whenever you discard a land card, create a Treasure token.
//! Whenever you discard a noncreature, nonland card, this creature fights up to
//! one target creature you don't control.
//!
//! (Scryfall "Treasure"/"Fight" are mechanic tags, not KeywordAbility variants
//! — keywords vec is empty.)
//!
//! All three triggers key on discarding a card OF A SPECIFIC TYPE. The
//! `CardDiscarded` trigger condition carries no card-type filter, and there is
//! no accessor for the discarded card's characteristics, so the discarded-card
//! TYPE GATE that distinguishes these three abilities is not expressible. Each
//! trigger is wired on the bare `CardDiscarded` condition, but its effect is
//! GAP'd (returns no effect) rather than firing indiscriminately on every
//! discard, which would be materially wrong.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surly Badgersaur");
    let badger = reg.interner_mut().intern("Badger");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(badger);
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: discard_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: discard_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: discard_noncreature_nonland,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn discard_creature(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you discard a CREATURE card" — CardDiscarded has no card-type filter
    // and no discarded-card accessor exists; the type gate is unexpressible.
    Vec::new()
}

fn discard_land(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you discard a LAND card" — type gate unexpressible (see above).
    Vec::new()
}

fn discard_noncreature_nonland(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you discard a NONCREATURE, NONLAND card" — type gate unexpressible.
    Vec::new()
}
