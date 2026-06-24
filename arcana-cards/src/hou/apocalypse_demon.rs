//! Apocalypse Demon — `{4}{B}{B}` */* Demon with Flying.
//!
//! Oracle:
//! * Flying — keyword.
//! * "Apocalypse Demon's power and toughness are each equal to the number of
//!   cards in your graveyard." — a characteristic-defining ability wired at
//!   Layer 7a via a SelfEntersBattlefield `self_pt_cda`: the compute reads the
//!   controller's graveyard size and sets base P/T to `(gy, gy)` (symmetric
//!   scalar).
//! * "At the beginning of your upkeep, tap this creature unless you sacrifice
//!   another creature." — an upkeep trigger. We model the baseline (tap this
//!   creature). The "unless you sacrifice another creature" escape is a non-
//!   mana / non-life optional payment (sacrifice is not an `OptionalPaymentKind`
//!   variant) — GAP'd; the creature taps unconditionally.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
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
    let name = reg.interner_mut().intern("Apocalypse Demon");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA — P/T each = cards in your graveyard — resolved at Layer 7a.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_tap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power and toughness each = number of cards in the controller's graveyard.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = script::graveyard_size(s, who) as i32;
    (n, n)
}

fn upkeep_tap(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "unless you sacrifice another creature" — sacrifice is not an
    // OptionalPaymentKind, so the escape clause cannot be modeled. Tap the
    // creature unconditionally (the baseline of the trigger).
    vec![Effect::Tap { target: trig.source }]
}
