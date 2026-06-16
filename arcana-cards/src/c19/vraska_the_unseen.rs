//! Vraska the Unseen — `{3}{B}{G}` Legendary Planeswalker — Vraska, starting
//! loyalty 5.
//!
//! Loyalty abilities:
//! * `+1`: Until your next turn, whenever a creature deals combat damage to
//!   Vraska, destroy that creature. GAP — the floating "until your next turn"
//!   combat-damage-to-source destroy rider is not expressible. Shell declared.
//! * `−3`: Destroy target nonland permanent.
//! * `−7`: Create three 1/1 black Assassin creature tokens with "Whenever this
//!   token deals combat damage to a player, that player loses the game."
//!   PARTIAL — the three tokens are wired; the "that player loses the game"
//!   triggered ability is a GAP (no lose-the-game Effect in the demonstrated
//!   surface), so the tokens are created without it (documented).

use arcana_core::effects::{Effect, TokenDefinition};
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
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vraska the Unseen");
    let vraska = reg.interner_mut().intern("Vraska");
    let _assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vraska);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, whenever a creature deals \
                       combat damage to Vraska, destroy that creature.".into(),
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
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target nonland permanent.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Create three 1/1 black Assassin creature tokens with \
                       \"Whenever this token deals combat damage to a player, \
                       that player loses the game.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ultimate_assassins,
            }),
    )
}

/// `+1`: combat-damage-to-source destroy rider.
fn plus_one(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "until your next turn, whenever a creature deals combat damage to
    // Vraska, destroy it" — floating combat-damage-to-source rider not
    // expressible.
    Vec::new()
}

/// `−3: Destroy target nonland permanent.`
fn minus_three_destroy(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `−7`: three 1/1 black Assassin tokens (lose-the-game trigger GAP'd).
fn ultimate_assassins(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let assassin = reg.interner().lookup("Assassin").expect("Assassin interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assassin);
    // GAP: the "that player loses the game" combat-damage trigger on each token
    // has no lose-the-game Effect in the demonstrated surface; tokens minted
    // without it.
    let token = TokenDefinition {
        name: assassin,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}
