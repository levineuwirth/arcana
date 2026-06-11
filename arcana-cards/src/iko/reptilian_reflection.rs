//! Reptilian Reflection — `{2}{R}` enchantment (Dominaria United,
//! 2022). "Whenever you cycle a card, you may have this enchantment
//! become a 5/4 Dinosaur creature with trample and haste in addition
//! to its other types until end of turn."
//!
//! No `TriggerCondition` observes cycling — wired on the closest,
//! `CardDiscarded { player: You }` (cycling pays its cost by
//! discarding), with a GAP for the over-fire on non-cycling discards.
//! The animation is AddType + SetBasePT + keyword grants until end of
//! turn; the Dinosaur subtype is a GAP (no add-subtype effect).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reptilian Reflection");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "Whenever you cycle a card" has no
                // TriggerCondition; CardDiscarded(You) is the closest
                // (cycling discards as its cost) but also fires on
                // non-cycling discards.
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: go_dino,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…become a 5/4 Dinosaur creature with trample and haste in
/// addition to its other types until end of turn." ("may" resolved as
/// animating; Dinosaur subtype GAP'd — no add-subtype effect.)
fn go_dino(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddType {
            target: trig.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: 5,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
