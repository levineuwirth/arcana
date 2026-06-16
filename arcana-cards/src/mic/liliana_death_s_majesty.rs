//! Liliana, Death's Majesty — `{3}{B}{B}` Legendary Planeswalker — Liliana.
//! Starting loyalty 5 (oracle).
//!
//! +1: Create a 2/2 black Zombie creature token. Mill two cards.
//! −3: Return target creature card from your graveyard to the battlefield.
//!     That creature is a black Zombie in addition to its other colors and
//!     types.
//!     GAP: graveyard-targeting loyalty ability (no any-graveyard target
//!     sentinel in the demonstrated surface), plus the becomes-a-black-Zombie
//!     rider on the returned card.
//! −7: Destroy all non-Zombie creatures.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

use arcana_core::effects::TokenDefinition;
use arcana_core::targets::ObjectFilter;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana, Death's Majesty");
    let liliana = reg.interner_mut().intern("Liliana");
    let _zombie = reg.interner_mut().intern("Zombie");
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
                text: "+1: Create a 2/2 black Zombie creature token. Mill two \
                       cards.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Return target creature card from your graveyard to \
                       the battlefield. It becomes a black Zombie.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Destroy all non-Zombie creatures.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_wrath,
            }),
    )
}

fn plus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg
        .interner()
        .lookup("Zombie")
        .expect("Zombie interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token },
        Effect::Mill { player: ctx.controller, count: 2 },
    ]
}

fn minus_three_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: graveyard-targeting reanimation (no any-graveyard target sentinel)
    //      plus the becomes-a-black-Zombie rider.
    Vec::new()
}

fn minus_seven_wrath(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Destroy all creatures that are not Zombies.
    let zombie_filter = script::subtype_filter(reg, "Zombie");
    let all_creatures = ObjectFilter::creature();
    let creatures = script::ids_matching(state, &all_creatures, ctx.controller);
    let zombies = script::ids_matching(state, &zombie_filter, ctx.controller);
    creatures
        .into_iter()
        .filter(|id| !zombies.contains(id))
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect()
}
