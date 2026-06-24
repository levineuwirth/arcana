//! Consuming Aberration — `{3}{U}{B}` */* Horror.
//! "Consuming Aberration's power and toughness are each equal to the
//! number of cards in your opponents' graveyards."
//! "Whenever you cast a spell, each opponent reveals cards from the top
//! of their library until they reveal a land card, then puts those cards
//! into their graveyard."
//!
//! The */* P/T (PtValue::Star) is backed by a self-CDA counting cards in
//! opponents' graveyards, wired at Layer 7a via
//! `ContinuousEffect::self_pt_cda` on an ETB trigger.
//! The SpellCast mill-until-land trigger is kept in shape, but its body
//! is GAP'd: RevealUntil routes the FOUND card only to hand/battlefield
//! (RevealDest), with no found→graveyard option, so the
//! everything-into-graveyard self-mill is inexpressible.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Consuming Aberration");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: opponents_mill_until_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Power and toughness are each equal to the number of cards in your
/// opponents' graveyards" — install the self-CDA at Layer 7a.
fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// P/T = total cards in all opponents' graveyards.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n: i32 = script::opponents(s, who)
        .into_iter()
        .map(|opp| s.objects.count_in_zone(Zone::Graveyard(opp)) as i32)
        .sum();
    (n, n)
}

fn opponents_mill_until_land(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each opponent reveals cards from the top of their library until they
    // reveal a land card, then puts those cards into their graveyard." —
    // RevealUntil's found card can only go to hand/battlefield (RevealDest), not
    // the graveyard, so routing the entire revealed run (land included) into the
    // graveyard is inexpressible.
    Vec::new()
}
