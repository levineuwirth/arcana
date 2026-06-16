//! Professor Onyx — `{4}{B}{B}` Legendary Planeswalker — Liliana,
//! starting loyalty 5.
//!
//! Magecraft — Whenever you cast or copy an instant or sorcery spell, each
//!   opponent loses 2 life and you gain 2 life.
//! +1: You lose 1 life. Look at the top three cards of your library. Put one
//!   of them into your hand and the rest into your graveyard.
//! −3: Each opponent sacrifices a creature with the greatest power among
//!   creatures that player controls.
//! −8: Each opponent may discard a card. If they don't, they lose 3 life.
//!   Repeat this process six more times.
//!
//! GAP: the Magecraft trigger fires only on CASTING an instant/sorcery
//!   (TriggerCondition::SpellCast); the "or copy" half has no corresponding
//!   trigger condition in the demonstrated surface.
//! GAP: −3 "each opponent sacrifices a creature with the GREATEST power among
//!   creatures that player controls" — the greatest-power selection is not
//!   expressible via the Sacrifice filter surface; ability shell declared
//!   with correct cost, effect GAP'd.
//! GAP: −8 repeated "may discard a card, else lose 3 life" seven times is a
//!   bespoke iterated may/else effect not in the demonstrated surface;
//!   ability shell declared with correct cost, effect GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Professor Onyx");
    let sub = reg.interner_mut().intern("Liliana");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Magecraft — whenever you cast an instant/sorcery: each opp loses 2, you gain 2.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: magecraft_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // +1: lose 1 life, dig top 3, one to hand, rest to graveyard.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You lose 1 life. Look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_dig,
            })
            // −3: each opponent sacrifices greatest-power creature (GAP)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Each opponent sacrifices a creature with the greatest power among creatures that player controls.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_sacrifice,
            })
            // −8: iterated may-discard-or-lose-3 (GAP)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Each opponent may discard a card. If they don't, they lose 3 life. Repeat this process six more times.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_iterate,
            }),
    )
}

fn magecraft_drain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::LoseLife { player: opp, amount: 2 })
        .collect();
    effects.push(Effect::GainLife { player: trig.controller, amount: 2 });
    effects
}

fn plus_one_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::LoseLife { player: ctx.controller, amount: 1 },
        Effect::DigTopN {
            player: ctx.controller,
            count: 3,
            filter: None,
            rest: arcana_core::effects::DigRest::Graveyard,
        },
    ]
}

fn minus_three_sacrifice(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "greatest power among creatures that player controls" selection
    // is not expressible via the Sacrifice filter surface.
    Vec::new()
}

fn minus_eight_iterate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: iterated may-discard-else-lose-3 (seven times) is a bespoke
    // effect not in the demonstrated surface.
    Vec::new()
}
