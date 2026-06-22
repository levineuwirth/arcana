//! Prosper, Tome-Bound — `{2}{B}{R}` 1/4 Legendary Tiefling Warlock.
//!
//! * Deathtouch.
//! * Mystic Arcanum — At the beginning of your end step, exile the top card
//!   of your library. Until the end of your next turn, you may play that card.
//! * Pact Boon — Whenever you play a card from exile, create a Treasure token.
//!
//! The Mystic Arcanum impulse maps to `Effect::ImpulseExile { count: 1 }`
//! (exile top card, may play it) though the printed window is "until end of
//! your next turn" while ImpulseExile is "until end of turn" — a fidelity
//! GAP. Pact Boon has no matching trigger condition (there is no "whenever
//! you play a card from exile" variant) and is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::{Phase, Step};
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prosper, Tome-Bound");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Mystic Arcanum — exile the top card; you may play it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: mystic_arcanum,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Pact Boon — Whenever you play a card from exile, create a Treasure.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: trigger — no "whenever you play a card from exile"
                //       TriggerCondition variant; using your-upkeep as a
                //       closest-firing placeholder, effect GAP'd to Vec::new.
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Ending,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pact_boon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn mystic_arcanum(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Fidelity GAP: printed window is "until the end of your next turn";
    // ImpulseExile grants play-permission only until end of turn.
    vec![Effect::ImpulseExile { player: trig.controller, count: 1 }]
}

fn pact_boon(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Whenever you play a card from exile, create a Treasure token" —
    //       no play-from-exile trigger condition exists; cannot fire on the
    //       correct event, so emit nothing rather than create Treasures wrongly.
    Vec::new()
}
