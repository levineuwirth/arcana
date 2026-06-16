//! Angrath, the Flame-Chained — `{3}{B}{R}` Legendary Planeswalker — Angrath, loyalty 6.
//!
//! +1: Each opponent discards a card and loses 2 life.
//! −3: Gain control of target creature until end of turn. Untap it. It gains
//!   haste until end of turn. Sacrifice it at the beginning of the next end step
//!   if it has mana value 3 or less.
//! −8: Each opponent loses life equal to the number of cards in their graveyard.
//!
//! # Scope
//! GAP: the −3 conditional "sacrifice it … if it has mana value 3 or less" rider
//!   is not modeled (no MV-gated delayed-sacrifice on a stolen permanent); the
//!   gain-control-EOT + untap + haste suite is faithful via ChangeControlEot.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angrath, the Flame-Chained");
    let angrath = reg.interner_mut().intern("Angrath");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angrath);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Each opponent discards a card and loses 2 life.".into(),
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
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Gain control of target creature until end of turn. Untap it. It gains haste until end of turn. Sacrifice it at the beginning of the next end step if it has mana value 3 or less.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Each opponent loses life equal to the number of cards in their graveyard.".into(),
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
                effect: minus_eight,
            }),
    )
}

fn plus_one(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::Discard {
            player: opp,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
        effects.push(Effect::LoseLife { player: opp, amount: 2 });
    }
    effects
}

fn minus_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice it if it has mana value 3 or less" rider not modeled.
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::ChangeControlEot { target: *id, new_controller: ctx.controller },
        Effect::Untap { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn minus_eight(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::opponents(state, ctx.controller)
        .into_iter()
        .map(|opp| Effect::LoseLife {
            player: opp,
            amount: script::graveyard_size(state, opp),
        })
        .collect()
}
