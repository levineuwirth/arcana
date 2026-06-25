//! Invasion of Gobakhan // Lightshield Array
//!
//! Front face: {1}{W} Battle — Siege with 4 defense counters.
//! ETB: look at target opponent's hand; you may exile a nonland card from it.
//! For as long as that card remains exiled, its owner may play it but it costs
//! {2} more.
//! Back face (Lightshield Array): Enchantment.
//! At the beginning of your end step, put a +1/+1 counter on each creature
//! that attacked this turn.
//! Sacrifice this enchantment: Creatures you control gain hexproof and
//! indestructible until end of turn.
//!
//! Defeat-transform to the enchantment back face is auto-wired by the engine SBA
//! (CR 310.11). The back face's end-step counter trigger and sacrifice activated
//! ability are wired (the latter face-gated to face 1).
//!
//! GAPs:
//! - ETB "look at hand / exile nonland / owner may play it for {2} more":
//!   the hand-look, optional exile-from-hand, and cost-rider are not expressible
//!   with any catalog Effect variant. Emitting Vec::new() for the ETB resolver.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Gobakhan");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Lightshield Array — Enchantment
    let back_name = reg.interner_mut().intern("Lightshield Array");
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    // ETB trigger targets a player (opponent)
    let etb_target = TargetRequirement {
        filter: TargetFilter::Player,
        count: TargetCount::Exactly(1),
        controller: Some(ControllerConstraint::Opponent),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_resolve,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![etb_target],
            })
            // Back face: "At the beginning of your end step, put a +1/+1 counter
            // on each creature that attacked this turn." Face-gated to face 1.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_counter_attackers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 1)
            // Back face: "Sacrifice this enchantment: Creatures you control gain
            // hexproof and indestructible until end of turn." Face-gated to face 1.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice this enchantment: Creatures you control gain hexproof and indestructible until end of turn.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1), // back (enchantment) face only
                effect: back_protect_creatures,
            })
            .with_transform_back(back_face),
    )
}

fn etb_resolve(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at target opponent's hand; you may exile a nonland card from
    // it; for as long as that card remains exiled, its owner may play it but
    // costs {2} more" — no catalog Effect for hand-look, exile-from-hand, or
    // play-permission with cost modification.
    Vec::new()
}

/// "Put a +1/+1 counter on each creature that attacked this turn." Enumerate
/// battlefield creatures and keep those that were declared as attackers.
fn back_counter_attackers(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::ids_matching(state, &ObjectFilter::creature(), trig.controller)
        .into_iter()
        .filter(|&id| script::creature_attacked_this_turn(state, id))
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        })
        .collect()
}

/// "Creatures you control gain hexproof and indestructible until end of turn."
/// Two filtered keyword grants over creatures the controller controls.
fn back_protect_creatures(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mine = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                ctx.source,
                mine.clone(),
                KeywordAbility::Hexproof,
                Duration::EndOfTurn,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                ctx.source,
                mine,
                KeywordAbility::Indestructible,
                Duration::EndOfTurn,
            ),
        },
    ]
}
