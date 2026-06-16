//! Liliana, Death's Majesty — `{3}{B}{B}` Legendary Planeswalker — Liliana, starting loyalty 5.
//!
//! (The Scryfall "Mill" keyword is the +1's "Mill two cards" action, not
//! a standalone keyword ability — emitted as Effect::Mill, keywords empty.)
//!
//! +1: Create a 2/2 black Zombie creature token. Mill two cards.
//!   IMPLEMENTED.
//! −3: Return target creature card from your graveyard to the battlefield.
//!   That creature is a black Zombie in addition to its other colors and
//!   types. GAP: "target card in YOUR graveyard" needs a concrete
//!   Zone::Graveyard(controller), and the controller id isn't known at
//!   register time (target_requirements are static); the additive "black
//!   Zombie" rider has no demonstrated add-color/add-subtype Effect
//!   either. Ability shell keeps the correct −3 cost, GAP'd body.
//! −7: Destroy all non-Zombie creatures. IMPLEMENTED via a ForEach
//!   destroying every battlefield creature lacking the Zombie subtype.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana, Death's Majesty");
    let liliana = reg.interner_mut().intern("Liliana");
    let zombie = reg.interner_mut().intern("Zombie");
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

    let _ = zombie;

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 2/2 black Zombie creature token. Mill two \
                       cards.".into(),
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
                effect: plus_one_zombie_mill,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Return target creature card from your graveyard to the \
                       battlefield. That creature is a black Zombie in addition to \
                       its other colors and types.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_reanimate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Destroy all non-Zombie creatures.".into(),
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
                effect: minus_seven_wrath,
            }),
    )
}

/// `+1` — create a 2/2 black Zombie token, then mill two.
fn plus_one_zombie_mill(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").expect("Zombie interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(zombie);
    vec![
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: zombie,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::Mill { player: ctx.controller, count: 2 },
    ]
}

/// `−3` — return target creature card from your graveyard; it's a black Zombie.
fn minus_three_reanimate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "your graveyard" needs Zone::Graveyard(controller) — the
    // controller id is dynamic, not knowable in static target_requirements
    // — and the additive "black Zombie" rider has no demonstrated Effect.
    Vec::new()
}

/// `−7` — destroy all non-Zombie creatures.
fn minus_seven_wrath(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").expect("Zombie interned");
    let filter = ObjectFilter::creature().without_subtype_sym(zombie);
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect()
}
