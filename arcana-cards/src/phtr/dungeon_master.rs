//! Dungeon Master — `{2}{W}{U}` Legendary Planeswalker — Dungeon Master,
//! starting loyalty 5. White-blue.
//!
//! Oracle:
//! +1: Target opponent creates a 1/1 black Skeleton creature token with "When
//!     this creature dies, each opponent gains 2 life."
//! +1: Roll a d20. If you roll a 1, skip your next turn. If you roll a 12 or
//!     higher, draw a card.
//! −6: You get an adventuring party. (Your party is a 3/3 red Fighter with
//!     first strike, a 1/1 white Cleric with lifelink, a 2/2 black Rogue with
//!     hexproof, and a 1/1 blue Wizard with flying.)
//!
//! # Scope
//! * +1 (Skeleton) — partial: target opponent creates a 1/1 black Skeleton
//!   token. GAP: the token's "when this dies, each opponent gains 2 life"
//!   internal ability (per-opponent dynamic) is omitted.
//! * +1 (d20) — GAP: rolling dice is not in the demonstrated Effect surface.
//! * −6 — GAP: "you get an adventuring party" is a bespoke multi-token effect.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dungeon Master");
    let sub = reg.interner_mut().intern("Dungeon Master");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);
    let _ = reg.interner_mut().intern("Skeleton");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Target opponent creates a 1/1 black Skeleton \
                       creature token with \"When this creature dies, each \
                       opponent gains 2 life.\"".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_skeleton,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Roll a d20. If you roll a 1, skip your next turn. If \
                       you roll a 12 or higher, draw a card.".into(),
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
                effect: plus_one_d20_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an adventuring party. (Your party is a 3/3 \
                       red Fighter with first strike, a 1/1 white Cleric with \
                       lifelink, a 2/2 black Rogue with hexproof, and a 1/1 \
                       blue Wizard with flying.)".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_gap,
            }),
    )
}

/// `+1:` target opponent creates a 1/1 black Skeleton token (die-trigger GAP'd).
fn plus_one_skeleton(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let skeleton = reg.interner().lookup("Skeleton").unwrap_or_default();
    let mut t_subtypes = SubtypeSet::default();
    t_subtypes.0.insert(skeleton);
    // GAP: token's "when this dies, each opponent gains 2 life" is omitted
    // (per-opponent dynamic life gain).
    let token = TokenDefinition {
        name: skeleton,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: t_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: *p,
        token,
    }]
}

/// `+1` (d20) — GAP: dice rolling not in Effect catalog.
fn plus_one_d20_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: roll a d20 (no dice-roll Effect surface).
    Vec::new()
}

/// `−6` — GAP: bespoke "adventuring party" multi-token effect.
fn minus_six_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you get an adventuring party" bespoke one-shot.
    Vec::new()
}
