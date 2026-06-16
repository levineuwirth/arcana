//! Ashiok, Wicked Manipulator — `{3}{B}{B}` Legendary Planeswalker —
//! Ashiok, starting loyalty 5. Black.
//!
//! Oracle text:
//! * If you would pay life while your library has at least that many
//!   cards in it, exile that many cards from the top of your library
//!   instead.
//! * `+1`: Look at the top two cards of your library. Exile one of them
//!   and put the other into your hand.
//! * `−2`: Create two 1/1 black Nightmare creature tokens with "At the
//!   beginning of combat on your turn, if a card was put into exile this
//!   turn, put a +1/+1 counter on this token."
//! * `−7`: Target player exiles the top X cards of their library, where
//!   X is the total mana value of cards you own in exile.
//!
//! # Scope
//!
//! * The static pay-life replacement is a bespoke replacement effect
//!   (not a loyalty ability) — not modeled.
//! * `+1`: "look at top two, exile one, put the other into hand" — the
//!   "exile the rest" disposition has no matching `DigRest` variant
//!   (only BottomRandom / Graveyard) — GAP'd.
//! * `−2` mints the two Nightmare tokens; their conditional combat
//!   trigger is a bespoke token ability — token body faithful, rider
//!   noted.
//! * `−7` has a dynamic exile count (X = total mana value in exile) —
//!   not expressible — GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ashiok, Wicked Manipulator");
    let ashiok = reg.interner_mut().intern("Ashiok");
    let _nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ashiok);

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
                text: "+1: Look at the top two cards of your library. Exile \
                       one of them and put the other into your hand.".into(),
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
                text: "−2: Create two 1/1 black Nightmare creature tokens \
                       with \"At the beginning of combat on your turn, if a \
                       card was put into exile this turn, put a +1/+1 \
                       counter on this token.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Target player exiles the top X cards of their \
                       library, where X is the total mana value of cards you \
                       own in exile.".into(),
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
                effect: minus_seven,
            }),
    )
}

/// `+1`: look at top two, exile one, put the other in hand.
fn plus_one(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile one, put the other into your hand" — the exile-the-rest
    // disposition has no matching DigRest variant (only BottomRandom /
    // Graveyard exist).
    Vec::new()
}

/// `−2`: create two 1/1 black Nightmare tokens (combat trigger omitted).
fn minus_two(_s: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // GAP (partial): the tokens' conditional combat-trigger ability isn't
    // built; the token bodies are faithful.
    let nightmare = reg.interner().lookup("Nightmare").expect("Nightmare interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    let token = TokenDefinition {
        name: nightmare,
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
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}

/// `−7`: target player exiles top X (X = mana value of your exiled cards).
fn minus_seven(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: dynamic exile count keyed off total mana value of cards you own
    // in exile — not expressible.
    Vec::new()
}
