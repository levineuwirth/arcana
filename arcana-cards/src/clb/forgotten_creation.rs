//! Forgotten Creation — `{3}{U}` 3/3 Creature — Zombie Horror.
//!
//! * Skulk (can't be blocked by creatures with greater power).
//! * At the beginning of your upkeep, you may discard all the cards in
//!   your hand. If you do, draw that many cards.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forgotten Creation");
    let zombie = reg.interner_mut().intern("Zombie");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Skulk],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: wheel_hand,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn wheel_hand(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "you may discard all the cards in your hand. If you do, draw that
    // many cards." — discard the whole hand, then draw the count that was
    // in hand at resolution. The "may" is the single discard-all choice.
    let n = script::hand_size(state, trig.controller);
    vec![Effect::Sequence(vec![
        Effect::Discard {
            player: trig.controller,
            count: n,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: trig.controller, count: n },
    ])]
}
