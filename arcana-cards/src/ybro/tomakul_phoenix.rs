//! Tomakul Phoenix — `{1}{R}{R}` 2/2 Phoenix with Flying and Haste.
//!
//! Oracle:
//! * Flying, haste.
//! * When Tomakul Phoenix dies, it perpetually gets +2/+2.
//! * At the beginning of combat on your turn, you may pay {X}{R}, where X is
//!   Tomakul Phoenix's power. If you do, return it from your graveyard to the
//!   battlefield.
//!
//! GAP: "perpetually gets +2/+2" — perpetual (Alchemy) modification has no
//! `Effect` variant; the dies-trigger effect is GAP'd.
//! GAP: the recursion trigger pays "{X}{R}, where X is this card's power" — a
//! variable-X mana payment; `OptionalPayment` accepts only a fixed `ManaCost`,
//! so the effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tomakul Phoenix");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_perpetual_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: combat_graveyard_recur,
                // Fires while in the graveyard.
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_perpetual_pump(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "perpetually gets +2/+2" — perpetual modification not modeled.
    Vec::new()
}

fn combat_graveyard_recur(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: pay "{X}{R}, where X is this card's power" — variable-X mana payment
    // is not expressible via OptionalPayment (fixed ManaCost only).
    Vec::new()
}
