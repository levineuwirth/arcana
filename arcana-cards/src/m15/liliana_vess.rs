//! Liliana Vess — `{3}{B}{B}` legendary planeswalker, starting loyalty 5.
//! Subtype Liliana; mono-black.
//!
//! Loyalty abilities:
//! * `+1`: Target player discards a card. (Functional via `Effect::Discard`.)
//! * `−2`: Search your library for a card, then shuffle and put that card
//!   on top. GAP — "put on top of library" ordering isn't expressible
//!   (`Effect::Search` destinations are bare zones with no top/bottom
//!   placement).
//! * `−8`: Put all creature cards from all graveyards onto the battlefield
//!   under your control. (Functional via `Effect::Reanimate` over every
//!   player's graveyard, controlled by you.)

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana Vess");
    let liliana = reg.interner_mut().intern("Liliana");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(liliana);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
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
                text: "+1: Target player discards a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_discard,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Search your library for a card, then shuffle and \
                       put that card on top."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Put all creature cards from all graveyards onto the \
                       battlefield under your control."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_reanimate,
            }),
    )
}

fn plus_one_discard(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}

fn minus_two_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "put on top of library" ordering isn't expressible (Search
    // destinations are bare zones with no top/bottom placement).
    Vec::new()
}

fn minus_eight_reanimate(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // Reanimate every player's graveyard creatures under your control.
    let effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Reanimate {
            player: ctx.controller,
            filter: ObjectFilter::creature(),
            from_zone: Zone::Graveyard(p),
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
