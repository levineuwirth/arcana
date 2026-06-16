//! Elspeth, Knight-Errant — `{2}{W}{W}` planeswalker, starting loyalty 4.
//! Legendary Planeswalker — Elspeth.
//!
//! Oracle text:
//! * `+1`: Create a 1/1 white Soldier creature token.
//! * `+1`: Target creature gets +3/+3 and gains flying until end of turn.
//! * `−8`: You get an emblem with "Artifacts, creatures, enchantments,
//!   and lands you control have indestructible."
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities (activated abilities whose cost is
//!   adding/removing loyalty counters).
//! * CR 704.5i — a planeswalker with 0 loyalty is sacrificed (SBA).
//!
//! # Scope
//!
//! Only the second `+1` ("target creature gets +3/+3 and gains flying
//! until end of turn") is fully modeled, via `Pump` + `GrantKeyword`.
//! The first `+1` creates a token (TokenDefinition builder not in the
//! demonstrated surface) and the `−8` grants an emblem — both have
//! their loyalty shells declared but GAP'd effect bodies.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elspeth, Knight-Errant");
    let elspeth = reg.interner_mut().intern("Elspeth");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elspeth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 white Soldier creature token.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Target creature gets +3/+3 and gains flying \
                       until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: You get an emblem with \"Artifacts, creatures, \
                       enchantments, and lands you control have \
                       indestructible.\"".into(),
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
                effect: ultimate_emblem,
            }),
    )
}

/// `+1: Create a 1/1 white Soldier creature token.`
fn plus_one_token(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token creation requires a TokenDefinition builder not in the
    // demonstrated surface.
    Vec::new()
}

/// `+1: Target creature gets +3/+3 and gains flying until end of turn.`
fn plus_one_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *id,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
    ]
}

/// `−8: You get an emblem with "...indestructible."`
fn ultimate_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation is not in the demonstrated Effect surface.
    Vec::new()
}
