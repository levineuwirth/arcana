//! Ashiok, Sculptor of Fears — `{4}{U}{B}` Legendary Planeswalker — Ashiok.
//! Colors B, U. Starting loyalty 5 (oracle).
//!
//! +2: Draw a card. Each player mills two cards.
//! −5: Put target creature card from a graveyard onto the battlefield under
//!     your control.
//!     GAP: graveyard-targeting loyalty ability (no any-graveyard target
//!     sentinel in the demonstrated surface).
//! −11: Gain control of all creatures target opponent controls.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

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
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-5: Put target creature card from a graveyard onto the \
                       battlefield under your control.".into(),
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
                effect: minus_five_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-11: Gain control of all creatures target opponent \
                       controls.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 11)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eleven_steal,
            }),
    )
}

fn plus_two(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }];
    for p in 0..state.num_players() {
        effects.push(Effect::Mill { player: p, count: 2 });
    }
    effects
}

fn minus_five_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: graveyard-targeting reanimation (no any-graveyard target sentinel).
    Vec::new()
}

fn minus_eleven_steal(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(target_player)) = ctx.targets.targets.first()
    else {
        return Vec::new();
    };
    state
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == *target_player && o.is_creature())
        .map(|o| Effect::ChangeControl {
            target: o.id,
            new_controller: ctx.controller,
        })
        .collect()
}
