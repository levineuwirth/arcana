//! Oko, the Trickster — `{4}{G}{U}` Legendary Planeswalker — Oko,
//! starting loyalty 5.
//!
//! +1: Put two +1/+1 counters on up to one target creature you control.
//! 0: Until end of turn, Oko becomes a copy of target creature you control.
//!    Prevent all damage that would be dealt to him this turn.
//! −7: Until end of turn, each creature you control has base power and
//!     toughness 10/10 and gains trample.
//!
//! # Scope
//! - The `+1` puts two +1/+1 counters on an optional target creature you
//!   control (modeled via two AddCounters).
//! - The `0` ("Oko becomes a copy of target creature you control") is a
//!   becomes-a-copy onto the source planeswalker — the demonstrated
//!   CopyPermanent variant mints a TOKEN copy rather than overwriting Oko's
//!   characteristics, so this is GAP'd (correct `0` cost + target shell kept).
//! - The `−7` sets each of your creatures to base 10/10 (board-wide
//!   filtered_set_base_pt) and grants trample (board-wide filtered_keyword),
//!   both until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oko, the Trickster");
    let oko = reg.interner_mut().intern("Oko");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(oko);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Put two +1/+1 counters on up to one target creature \
                       you control."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_counters,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Until end of turn, Oko becomes a copy of target creature \
                       you control. Prevent all damage that would be dealt to him \
                       this turn."
                    .into(),
                cost: ActivationCost::default(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_become_copy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Until end of turn, each creature you control has base \
                       power and toughness 10/10 and gains trample."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_tenten,
            }),
    )
}

/// `+1: Put two +1/+1 counters on up to one target creature you control.`
fn plus_one_counters(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}

/// `0: Oko becomes a copy of target creature you control.`
fn zero_become_copy(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "becomes a copy of target creature" overwrites the source
    // planeswalker's characteristics — the CopyPermanent variant mints a token
    // copy rather than re-skinning Oko, and there's no becomes-a-copy-onto-self
    // primitive in the demonstrated surface. Correct `0` cost + target retained.
    Vec::new()
}

/// `−7: Each creature you control has base P/T 10/10 and gains trample.`
fn minus_seven_tenten(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // Board-wide: every creature you control becomes base 10/10 and gains
    // trample until end of turn (re-evaluated each layer pass, so creatures
    // entering this turn are also affected).
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_set_base_pt(
                ctx.source,
                filter.clone(),
                10,
                10,
                Duration::EndOfTurn,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                ctx.source,
                filter,
                KeywordAbility::Trample,
                Duration::EndOfTurn,
            ),
        },
    ]
}
