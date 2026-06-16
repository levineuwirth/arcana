//! Edgar Markov — `{3}{R}{W}{B}` 4/4 Legendary Vampire Knight with
//! First strike and Haste.
//!
//! * Eminence — Whenever you cast another Vampire spell, if Edgar is in
//!   the command zone or on the battlefield, create a 1/1 black Vampire
//!   creature token. (Wired as a filtered `SpellCast` trigger; the
//!   command-zone half of the intervening-if is unexpressible — the
//!   `conditions` helpers don't reach the command zone — so the gate is
//!   left `None` and the battlefield case is covered.)
//! * First strike, haste — base keywords.
//! * Whenever Edgar attacks, put a +1/+1 counter on each Vampire you
//!   control.
//!
//! `Eminence` itself is not a usable `KeywordAbility` variant, so it is
//! not listed in `keywords`; its rules are modeled by the trigger.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Edgar Markov");
    let vampire = reg.interner_mut().intern("Vampire");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(knight);

    let vampire_filter = script::subtype_filter(reg, "Vampire");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(vampire_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: eminence_make_vampire,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_counter_each_vampire,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Eminence: create a 1/1 black Vampire creature token.
fn eminence_make_vampire(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: arcana_core::effects::TokenDefinition {
            name: vampire,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// Attack trigger: put a +1/+1 counter on each Vampire you control.
fn attacks_counter_each_vampire(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Vampire").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}
