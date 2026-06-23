//! Nicol Bolas — `{2}{U}{U}{B}{B}{R}{R}` 7/7 Legendary Creature — Elder Dragon.
//!
//! Flying.
//! At the beginning of your upkeep, sacrifice Nicol Bolas unless you pay
//! {U}{B}{R}.
//! Whenever Nicol Bolas deals damage to an opponent, that player discards
//! their hand.
//!
//! # Decomposition
//! * Keyword line — `Flying`.
//! * "At the beginning of your upkeep, sacrifice … unless you pay {U}{B}{R}"
//!   → upkeep trigger (id 1) wrapping an `OptionalPayment`: on no-pay the
//!   source is sacrificed (routed through `DestroyPermanent`, the engine's
//!   documented sacrifice routing).
//! * "Whenever Nicol Bolas deals damage to an opponent, that player discards
//!   their hand" → a damage-dealt trigger (id 2) on the damaged player.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nicol Bolas");
    let elder = reg.interner_mut().intern("Elder");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dragon);
    // Source filter restricting the damage trigger to Nicol Bolas itself.
    let self_source = ObjectFilter {
        name: Some(name),
        ..ObjectFilter::default()
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}{B}{B}{R}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: sacrifice_unless_pay,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: self_source,
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: opponent_discards_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn sacrifice_unless_pay(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{U}{B}{R}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DestroyPermanent { target: trig.source })),
    }]
}

fn opponent_discards_hand(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    // Restrict to "an opponent": skip if the damaged player is the controller.
    if p == trig.controller {
        return Vec::new();
    }
    let n = script::hand_size(state, p);
    vec![Effect::Discard {
        player: p,
        count: n,
        choice: DiscardChoice::ControllerChooses,
    }]
}
