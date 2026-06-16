//! Chandra, Flameshaper — `{5}{R}{R}` Legendary Planeswalker — Chandra.
//! Printed starting loyalty 5 (CR 113.3c).
//!
//! Loyalty abilities (CR 606):
//! * `+2`: Add {R}{R}{R}. Exile the top three cards of your library;
//!   choose one, you may play that card this turn.
//! * `+1`: Create a token that's a copy of target creature you control,
//!   except it has haste and "At the beginning of the end step,
//!   sacrifice this token."
//! * `−4`: Chandra deals 8 damage divided as you choose among any
//!   number of target creatures and/or planeswalkers.
//!
//! Scope: the `+2` is split — the `Add {R}{R}{R}` mana is expressible
//! and emitted; the "exile top three, choose one, may play it" rider is
//! a GAP (no choose-one-of-exiled + cast-from-exile primitive in the
//! demonstrated surface). The `+1` copies via `Effect::CopyPermanent`
//! (the haste + sacrifice-EOT rider on the copy is not expressible
//! through CopyPermanent — GAP'd). The `−4` is divided damage via
//! `Effect::DealDamageDivided`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Flameshaper");
    let chandra = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Add {R}{R}{R}. Exile the top three cards of your \
                       library. Choose one. You may play that card this turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_ritual,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a token that's a copy of target creature you \
                       control, except it has haste and \"At the beginning of the \
                       end step, sacrifice this token.\"".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_copy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−4: Chandra deals 8 damage divided as you choose among any \
                       number of target creatures and/or planeswalkers.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::Any,
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_divided,
            }),
    )
}

/// `+2`: the mana half is expressible; the impulse-exile-and-choose-one
/// rider is a GAP.
fn plus_two_ritual(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile top three, choose one, you may play it this turn" needs a
    // choose-one-of-exiled + cast-from-exile primitive not in the surface.
    // The Add {R}{R}{R} half is emitted.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
        ],
    }]
}

/// `+1`: copy a creature you control.
fn plus_one_copy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: the "except it has haste and sacrifice-at-end-step" rider on
    // the copy isn't expressible through CopyPermanent. Plain copy emitted.
    vec![Effect::CopyPermanent { target: *id }]
}

/// `−4`: deal 8 damage divided among the chosen targets.
fn minus_four_divided(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets: Vec<DamageTarget> = ctx
        .targets
        .targets
        .iter()
        .filter_map(|c| match c {
            TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
            _ => None,
        })
        .collect();
    if targets.is_empty() {
        return Vec::new();
    }
    vec![Effect::DealDamageDivided {
        source: ctx.source,
        targets,
        total: 8,
    }]
}
