//! The Eternal Wanderer — `{4}{W}{W}` Legendary Planeswalker.
//! Printed starting loyalty 5 (CR 113.3c).
//!
//! Static: "No more than one creature can attack The Eternal Wanderer
//! each combat." — a combat-restriction static, not a loyalty ability;
//! GAP'd (no demonstrated surface for per-combat attacker caps on a PW).
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Exile up to one target artifact or creature; return that card
//!   to the battlefield at the beginning of that player's next end step.
//! * `0`: Create a 2/2 white Samurai creature token with double strike.
//! * `−4`: For each player, choose a creature that player controls. Each
//!   player sacrifices all creatures not chosen this way.
//!
//! Scope: `+1` modeled as a blink — `Effect::ExilePermanent` + the
//! return-at-next-end-step half is GAP'd (the chosen object is re-id'd
//! by the exile move, so a return target isn't recoverable from the
//! resolver; demonstrated DelayedAction blink primitives act on a known
//! id only). `0` is fully expressible. `−4` "keep one creature per
//! player, sacrifice the rest" has no demonstrated surface — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Eternal Wanderer");
    let wanderer = reg.interner_mut().intern("Wanderer");
    let _samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wanderer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };
    // GAP: "No more than one creature can attack The Eternal Wanderer each
    // combat" is a continuous combat restriction with no demonstrated surface.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Exile up to one target artifact or creature. Return \
                       that card to the battlefield under its owner's control at \
                       the beginning of that player's next end step.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_blink,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Create a 2/2 white Samurai creature token with double \
                       strike.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_samurai,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−4: For each player, choose a creature that player controls. \
                       Each player sacrifices all creatures they control not chosen \
                       this way.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_cull,
            }),
    )
}

/// `+1`: exile up to one target; the return rider is a GAP.
fn plus_one_blink(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        // "up to one" — choosing zero is a clean no-op.
        return Vec::new();
    };
    // GAP: "return at the beginning of that player's next end step" — the
    // exiled object is re-id'd by the zone move; no return-by-known-id
    // delayed-blink primitive recovers the new id from this resolver.
    vec![Effect::ExilePermanent { target: *id }]
}

/// `0`: create the 2/2 double-strike Samurai.
fn zero_samurai(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let samurai = reg
        .interner()
        .lookup("Samurai")
        .expect("Samurai interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(samurai);
    let token = TokenDefinition {
        name: samurai,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}

/// `−4`: "keep one creature per player, sacrifice the rest" — GAP.
fn minus_four_cull(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: per-player "choose one to keep, sacrifice all others" has no
    // demonstrated surface (no keep-N / inverse-sacrifice selection).
    Vec::new()
}
