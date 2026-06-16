//! Liliana, Waker of the Dead — `{2}{B}{B}` Legendary Planeswalker — Liliana, starting loyalty 5.
//!
//! +1: Each player discards a card. Each opponent who can't loses 3 life.
//!   PARTIAL: "each player discards a card" IMPLEMENTED (per-player
//!   Discard). GAP: the "each opponent who can't loses 3 life" rider is
//!   conditional on a failed discard, which the demonstrated Effect
//!   surface can't express.
//! −3: Target creature gets -X/-X until end of turn, where X is the
//!   number of cards in your graveyard. IMPLEMENTED via a resolution-time
//!   Pump with negative power/toughness read from graveyard size.
//! −7: You get an emblem with "At the beginning of combat on your turn,
//!   put target creature card from a graveyard onto the battlefield under
//!   your control. It gains haste." GAP: graveyard-targeted reanimation
//!   needs a concrete Zone::Graveyard sentinel the demonstrated surface
//!   lacks; the emblem shell is still created.

use arcana_core::effects::{DiscardChoice, Effect, EmblemDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana, Waker of the Dead");
    let liliana = reg.interner_mut().intern("Liliana");
    let _emblem = reg.interner_mut().intern("Liliana, Waker of the Dead emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(liliana);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Each player discards a card. Each opponent who can't \
                       loses 3 life.".into(),
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
                effect: plus_one_discard,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Target creature gets -X/-X until end of turn, where X \
                       is the number of cards in your graveyard.".into(),
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
                effect: minus_three_shrink,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"At the beginning of combat on \
                       your turn, put target creature card from a graveyard onto \
                       the battlefield under your control. It gains haste.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

/// `+1` — each player discards a card.
fn plus_one_discard(
    state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each opponent who can't loses 3 life" is conditional on a
    // failed discard — not expressible. The discard itself is emitted.
    (0..state.num_players())
        .map(|p| Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        })
        .collect()
}

/// `−3` — target creature gets -X/-X (X = cards in your graveyard).
fn minus_three_shrink(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let x = script::graveyard_size(state, ctx.controller) as i32;
    vec![Effect::Pump {
        target: *id,
        power: -x,
        toughness: -x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

/// `−7` — emblem (combat-trigger graveyard reanimation).
fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Liliana, Waker of the Dead emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // GAP: "put target creature card from a graveyard onto the
            // battlefield" needs a concrete Zone::Graveyard target the
            // demonstrated surface can't express. Emblem shell created.
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
