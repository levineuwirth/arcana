//! Slicer, Hired Muscle // Slicer, High-Speed Antagonist
//!
//! Front: Legendary Artifact Creature — Robot 3/4. Double strike, Haste.
//! More Than Meets the Eye {2}{R} (alternate cast cost — GAP: not modeled).
//! At the beginning of each opponent's upkeep, you may have that player gain control of Slicer
//! until end of turn. If you do, untap Slicer, goad it, and it can't be sacrificed this turn.
//! If you don't, convert it.
//! Back: Legendary Artifact — Vehicle. Living metal. First strike, Haste.
//! Whenever Slicer deals combat damage to a player, convert it at end of combat.
//!
//! GAP: More Than Meets the Eye alternate cast cost not modeled.
//! GAP: "you may have that player gain control until end of turn" (conditional control transfer
//!      at opponent's upkeep) — the OptionalPayment shape isn't suited to this opponent trigger;
//!      partial: ChangeControlEot, untap, goad emitted without "can't be sacrificed" rider.
//! GAP: "can't be sacrificed this turn" not expressible.
//! GAP: "if you don't, convert it" in the else branch of the upkeep trigger — GAP.
//! GAP: Living metal not modeled.
//! GAP: "deals combat damage to a player, convert at end of combat" — back-face trigger GAP.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::targets::ControllerConstraint;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slicer, Hired Muscle");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::DoubleStrike, KeywordAbility::Haste],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Slicer, High-Speed Antagonist");
    let vehicle_sub = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine(TypeLine::ARTIFACT),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // GAP: Living metal not modeled
            keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face: at the beginning of each opponent's upkeep, effect on Slicer
            // GAP: conditional "if you do / if you don't" with ChangeControlEot is not fully
            // expressible. Partial: goad only (opponent gets creature, untap, goad).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: arcana_core::turn::Phase::PreCombatMain,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: upkeep_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may have that player gain control" — conditional; full goad+control not expressible
    // Partial: goad Slicer (can't attack its controller, must attack each combat)
    vec![
        Effect::Goad {
            target: trig.source,
            goader: trig.controller,
            duration: Duration::EndOfTurn,
        },
        Effect::Untap { target: trig.source },
    ]
}
