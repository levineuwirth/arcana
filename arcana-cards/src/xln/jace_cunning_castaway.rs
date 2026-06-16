//! Jace, Cunning Castaway — `{1}{U}{U}` Legendary Planeswalker — Jace,
//! starting loyalty 3.
//!
//! +1: Whenever one or more creatures you control deal combat damage to a
//!     player this turn, draw a card, then discard a card. GAP: a one-turn
//!     floating delayed combat-damage trigger is not expressible from a
//!     loyalty resolver.
//! −2: Create a 2/2 blue Illusion creature token with "When this token becomes
//!     the target of a spell, sacrifice it." (Token minted; the
//!     becomes-targeted self-sacrifice ability is GAP.)
//! −5: Create two tokens that are copies of Jace, except they're not
//!     legendary. GAP: token copies of a planeswalker (self) are bespoke.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
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
    let name = reg.interner_mut().intern("Jace, Cunning Castaway");
    let jace = reg.interner_mut().intern("Jace");
    let _illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Whenever one or more creatures you control deal combat \
                       damage to a player this turn, draw a card, then discard a \
                       card.".into(),
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
                text: "-2: Create a 2/2 blue Illusion creature token with \"When \
                       this token becomes the target of a spell, sacrifice it.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-5: Create two tokens that are copies of Jace, except \
                       they're not legendary.".into(),
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
                effect: minus_five,
            }),
    )
}

fn plus_one(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: floating one-turn combat-damage delayed trigger.
    Vec::new()
}

fn minus_two(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "when this token becomes the target of a spell, sacrifice it" — the
    // token is minted without that triggered ability.
    let illusion = reg.interner().lookup("Illusion")
        .expect("Illusion interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(illusion);
    let token = TokenDefinition {
        name: illusion,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_five(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: two non-legendary token copies of Jace himself.
    Vec::new()
}
