//! Ashiok, Sculptor of Fears — `{4}{U}{B}` Legendary Planeswalker — Ashiok,
//! starting loyalty 5.
//!
//! +2: Draw a card. Each player mills two cards.
//! −5: Put target creature card from a graveyard onto the battlefield under
//!     your control.
//! −11: Gain control of all creatures target opponent controls.
//!
//! # Scope
//! - The `+2` draws one then mills two for each player (iterated via
//!   script::all_players).
//! - The `−5` reanimates a target creature card from a graveyard. Targeting a
//!   card in a graveyard needs a concrete `Zone::Graveyard(player)` and there's
//!   no any-graveyard target sentinel in the demonstrated surface, so this is
//!   GAP'd (correct `−5` cost shell retained; no target requirement declared).
//! - The `−11` gains control (permanently) of every creature the targeted
//!   opponent controls — script the opponent's creatures, ChangeControl each.
//!
//! GAP: Scryfall tagged "Mill" — the mill lives inside the `+2` loyalty
//! ability, so no card-level keyword is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ashiok, Sculptor of Fears");
    let ashiok = reg.interner_mut().intern("Ashiok");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ashiok);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Draw a card. Each player mills two cards.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_draw_mill,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-5: Put target creature card from a graveyard onto the \
                       battlefield under your control."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_reanimate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-11: Gain control of all creatures target opponent \
                       controls."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 11)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eleven_gain_control,
            }),
    )
}

/// `+2: Draw a card. Each player mills two cards.`
fn plus_two_draw_mill(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::DrawCards { player: ctx.controller, count: 1 }];
    for p in script::all_players(state) {
        effects.push(Effect::Mill { player: p, count: 2 });
    }
    effects
}

/// `−5: Put target creature card from a graveyard onto the battlefield.`
fn minus_five_reanimate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: targeting a creature card in a graveyard needs a concrete
    // Zone::Graveyard(player) and there's no any-graveyard target sentinel in
    // the demonstrated surface. Correct `−5` cost shell retained.
    Vec::new()
}

/// `−11: Gain control of all creatures target opponent controls.`
fn minus_eleven_gain_control(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(opp)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Player(*opp));
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::ChangeControl {
            target: id,
            new_controller: ctx.controller,
        })
        .collect()
}
