//! God-Eternal Rhonas — `{3}{G}{G}` 5/5 Legendary Zombie God with Deathtouch.
//!
//! "Deathtouch
//!  When God-Eternal Rhonas enters, double the power of each other creature you
//!  control until end of turn. Those creatures gain vigilance until end of turn.
//!  When God-Eternal Rhonas dies or is put into exile from the battlefield, you
//!  may put it into its owner's library third from the top."
//!
//! Deathtouch is a base keyword. The ETB trigger has two parts: doubling power
//! (a per-creature multiplicative pump — `Effect::Pump` is additive only and no
//! "double power" Effect exists, so the doubling is GAP'd) and granting
//! vigilance until end of turn, which IS expressible via `Effect::ForEach` over
//! every other creature you control + `Effect::GrantKeyword`.
//!
//! The leaves-the-battlefield trigger ("dies or is put into exile … put it into
//! its owner's library third from the top") has no expressible effect — there
//! is no "put into library third from top" Effect — so that whole ability is
//! GAP'd.

// GAP (trigger 1, part): "double the power of each other creature you control" —
// no multiplicative / double-power Effect (Pump is additive).
// GAP (trigger 2): "dies or is put into exile … put it into its owner's library
// third from the top" — no "into library Nth from top" Effect, and no single
// dies-or-exiled trigger condition.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("God-Eternal Rhonas");
    let zombie = reg.interner_mut().intern("Zombie");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_grant_vigilance,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_tuck,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_grant_vigilance(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Those creatures gain vigilance until end of turn" — each OTHER creature
    // you control. (The power-doubling half is GAP'd; see header.)
    let ids: Vec<_> = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    )
    .into_iter()
    .filter(|id| *id != trig.source)
    .collect();
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        }),
    }]
}

fn dies_tuck(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "put it into its owner's library third from the top" — no Effect for
    // a specific library position.
    Vec::new()
}
