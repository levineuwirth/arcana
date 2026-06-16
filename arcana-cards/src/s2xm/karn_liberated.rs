//! Karn Liberated — `{7}` Legendary Planeswalker — Karn,
//! starting loyalty 6 — colorless.
//!
//! Oracle text:
//! * `+4`: Target player exiles a card from their hand. — the targeted player
//!   chooses a card from their OWN hand to exile; there is no Effect for
//!   "target player exiles a card from their hand" (Discard goes to graveyard,
//!   not exile, and the chooser is the targeted player). GAP.
//! * `−3`: Exile target permanent. — `ExilePermanent`.
//! * `−14`: Restart the game, leaving in exile all non-Aura permanent cards
//!   exiled with Karn. Then put those cards onto the battlefield under your
//!   control. — restarting the game is not expressible. GAP.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karn Liberated");
    let karn = reg.interner_mut().intern("Karn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(karn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+4: Target player exiles a card from their hand.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_four_exile_hand,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Exile target permanent.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_exile,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−14: Restart the game, leaving in exile all non-Aura \
                       permanent cards exiled with Karn Liberated. Then put \
                       those cards onto the battlefield under your control."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 14)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_fourteen_restart,
            }),
    )
}

/// `+4`: target player exiles a card from their hand.
fn plus_four_exile_hand(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target player exiles a card from their hand" — the targeted player
    // chooses from their own hand, and exile-from-hand is not expressible
    // (Discard moves to graveyard, not exile).
    Vec::new()
}

/// `−3`: exile target permanent.
fn minus_three_exile(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let target = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    vec![Effect::ExilePermanent { target }]
}

/// `−14`: restart the game.
fn minus_fourteen_restart(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: restarting the game is not expressible from the demonstrated
    // Effect surface.
    Vec::new()
}
