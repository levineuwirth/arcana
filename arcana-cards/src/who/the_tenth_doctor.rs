//! The Tenth Doctor — `{3}{U}{R}` 3/5 Legendary Time Lord Doctor (U/R).
//! Allons-y! — Whenever you attack, exile cards from the top of your library
//!   until you exile a nonland card. Put three time counters on it. If it
//!   doesn't have suspend, it gains suspend.
//! Timey-Wimey — {7}: Time travel three times. Activate only as a sorcery.
//!
//! "Allons-y!", "Time Travel", and "Timey-Wimey" are ability words / unmodeled
//! keywords (not in the supported keyword surface). The attack trigger is
//! approximated as CreatureAttacks (your creatures), but its effect — exile
//! until a nonland, stack time counters on the exiled card, grant suspend — is
//! not expressible, so it is GAP'd. The {7} activated cost is expressible but
//! "time travel" has no effect primitive, so that effect is GAP'd too.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Tenth Doctor");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let doctor = reg.interner_mut().intern("Doctor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(doctor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Allons-y! / Time Travel / Timey-Wimey are ability words /
        // unmodeled keywords, not in the supported keyword surface.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "Whenever you attack" approximated as a creature you control attacking.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: allons_y,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{7}: Time travel three times. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{7}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: timey_wimey,
            }),
    )
}

fn allons_y(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile cards from the top of your library until you exile a nonland
    // card, put three time counters on it, and if it doesn't have suspend it
    // gains suspend." RevealUntil can only send the found card to hand or the
    // battlefield (not exile-with-counters), and suspend/time-counter stacking
    // on an exiled card is not expressible.
    Vec::new()
}

fn timey_wimey(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Time travel three times" has no effect primitive (suspend / time
    // counter manipulation across the board is unmodeled).
    Vec::new()
}
