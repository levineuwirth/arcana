//! Dream Strix — `{2}{U}` 3/2 blue Bird Illusion.
//! Flying.
//! When this creature becomes the target of a spell, sacrifice it.
//! When this creature dies, learn.
//!
//! Flying is a base keyword. The "becomes the target of a spell" trigger
//! maps to SelfBecomesTarget; sacrificing self is scheduled as a delayed
//! sacrifice on this dies-no-wait — actually applied via DelayedAction::Sacrifice
//! scheduled on ThisDies is wrong; we sacrifice immediately by destroying.
//! The dies "learn" trigger has no engine primitive (Lesson/outside-the-game)
//! and is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dream Strix");
    let bird = reg.interner_mut().intern("Bird");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // When this creature becomes the target of a spell, sacrifice it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: sacrifice_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // When this creature dies, learn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_learn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Sacrifice (destroy) this creature when it becomes the target of a spell.
fn sacrifice_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DestroyPermanent { target: trig.source }]
}

/// "Learn" — reveal a Lesson card from outside the game OR discard to draw.
/// Neither half is expressible (no outside-the-game zone primitive).
fn dies_learn(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Learn (reveal a Lesson card from outside the game, or discard a
    // card to draw a card) — no engine primitive for the outside-the-game /
    // sideboard zone or the Learn choice.
    Vec::new()
}
