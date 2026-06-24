//! Opulent Clomper — `{1}{G}` */* Ouphe.
//!
//! Oracle:
//! * "Vivid — Opulent Clomper's power and toughness are each equal to the
//!   number of colors among permanents you control." — a characteristic-
//!   defining ability wired at Layer 7a via a SelfEntersBattlefield
//!   `self_pt_cda`: the compute ORs the base color bits of every permanent the
//!   controller controls, counts the distinct colors `n`, and sets base P/T to
//!   `(n, n)` (symmetric scalar). Reads BASE colors (recursion-proof).
//! * "At the beginning of your upkeep, if this creature isn't all colors, it
//!   becomes a random color that it isn't in addition to its other colors." —
//!   GAP: there is no random-color-addition effect primitive.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Opulent Clomper");
    let ouphe = reg.interner_mut().intern("Ouphe");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ouphe);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA — P/T each = number of colors among permanents you control.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
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
                // The "if this creature isn't all colors" gate is part of the
                // GAP'd random-color effect, not expressed as an intervening_if.
                intervening_if: None,
                effect: upkeep_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "At the beginning of your upkeep, if this creature isn't all colors, it
/// becomes a random color that it isn't in addition to its other colors."
fn upkeep_gap(_s: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: there is no random-color-addition effect primitive (nor an "isn't
    // all colors" continuous condition to gate it on).
    Vec::new()
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

// Power and toughness each = number of distinct colors among the controller's
// permanents (OR of base color bits, count set bits).
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let mut bits: u8 = 0;
    for o in s.objects.objects_in_zone(Zone::Battlefield) {
        if o.controller == who {
            bits |= o.characteristics.colors.0;
        }
    }
    let n = bits.count_ones() as i32;
    (n, n)
}
