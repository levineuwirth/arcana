//! Mordenkainen — `{4}{U}{U}` Legendary Planeswalker — Mordenkainen,
//! starting loyalty 5.
//!
//! +2: Draw two cards, then put a card from your hand on the bottom of your
//!   library. The draw-two is modeled; the "put a card from your hand on the
//!   bottom" rider (a hand-selection bottoming) is not expressible —
//!   partially GAP'd.
//! −2: Create a blue Dog Illusion creature token with a power/toughness each
//!   equal to twice the number of cards in your hand. The token is created
//!   as a blue Dog Illusion; its dynamic "*/*" (twice your hand size)
//!   power/toughness is not expressible from `TokenDefinition` — partially
//!   GAP'd (created 0/0; PtValue has no hand-count form here).
//! −10: Exchange your hand and library, then shuffle. You get an emblem with
//!   "You have no maximum hand size." Both the hand/library exchange and the
//!   "no maximum hand size" rule-altering static emblem are not expressible.
//!   GAP (correct −10 cost shell).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mordenkainen");
    let mordenkainen = reg.interner_mut().intern("Mordenkainen");
    let _dog = reg.interner_mut().intern("Dog");
    let _illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mordenkainen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Draw two cards, then put a card from your hand on the \
                       bottom of your library.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create a blue Dog Illusion creature token with \"This \
                       token's power and toughness are each equal to twice the \
                       number of cards in your hand.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: Exchange your hand and library, then shuffle. You get \
                       an emblem with \"You have no maximum hand size.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_exchange,
            }),
    )
}

fn plus_two_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then put a card from your hand on the bottom of your library" —
    //      a hand-selection bottoming rider not expressible here; draw-two
    //      is modeled.
    vec![Effect::DrawCards { player: ctx.controller, count: 2 }]
}

fn minus_two_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the token's dynamic "*/*" (twice the number of cards in your hand)
    //      power/toughness is not expressible from TokenDefinition; the blue
    //      Dog Illusion token is created with fixed 0/0.
    let dog = reg.interner().lookup("Dog").expect("Dog interned at register");
    let illusion = reg
        .interner()
        .lookup("Illusion")
        .expect("Illusion interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(dog);
    token_subtypes.0.insert(illusion);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: dog,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn minus_ten_exchange(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Exchange your hand and library, then shuffle" has no expressible
    //      primitive, and the "You have no maximum hand size" emblem is a
    //      rule-altering static the emblem builders can't express.
    Vec::new()
}
