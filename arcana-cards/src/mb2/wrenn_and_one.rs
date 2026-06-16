//! Wrenn and One — Land Planeswalker — Wrenn.
//! Starting loyalty inferred 3.
//! +1: Wrenn and One gains "{T}: Add {G}" until your next turn.
//! −1: Create a 1/1 green Squirrel creature token.
//! −4: You get an emblem with "At the beginning of your precombat main phase,
//!   add {G} for each creature you control."
//!
//! GAP: +1 grants a temporary activated mana ability ("{T}: Add {G}" until
//!   your next turn) — there is no grant-an-activated-ability effect in the
//!   demonstrated surface. Declared with the correct +1 cost, effect GAP'd.
//! GAP: −4 emblem carries a triggered mana ability whose effect is a dynamic
//!   per-creature mana add; not expressible as a self-contained EmblemDefinition
//!   trigger here. Declared with the correct −4 cost, effect GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrenn and One");
    let wrenn = reg.interner_mut().intern("Wrenn");
    let _squirrel = reg.interner_mut().intern("Squirrel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wrenn);

    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::LAND | TypeLine::PLANESWALKER),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Wrenn and One gains \"{T}: Add {G}\" until your next turn.".into(),
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
                effect: plus_one_grant_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Create a 1/1 green Squirrel creature token.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: You get an emblem with \"At the beginning of your precombat \
                       main phase, add {G} for each creature you control.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_emblem,
            }),
    )
}

fn plus_one_grant_mana(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: grant a temporary activated mana ability — no such effect.
    Vec::new()
}

fn minus_one_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let squirrel = reg.interner().lookup("Squirrel").expect("Squirrel interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(squirrel);
    let token = TokenDefinition {
        name: squirrel,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_four_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem's dynamic per-creature mana trigger not expressible here.
    Vec::new()
}
