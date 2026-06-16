//! Karn, Scion of Urza — `{4}` Legendary Planeswalker — Karn,
//! starting loyalty 5. Colorless.
//!
//! +1: Reveal the top two cards of your library. An opponent chooses one of
//!     them. Put that card into your hand and exile the other with a silver
//!     counter on it. GAP: opponent-choice reveal + silver-counter exile is a
//!     bespoke effect with no demonstrated primitive.
//! −1: Put a card you own with a silver counter on it from exile into your
//!     hand. GAP: exile-by-counter retrieval is bespoke.
//! −2: Create a 0/0 colorless Construct artifact creature token with "This
//!     token gets +1/+1 for each artifact you control."
//!     GAP: the token's self-pump CDA is not expressible; we mint a 0/0
//!          colorless Construct artifact creature token.

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
    let name = reg.interner_mut().intern("Karn, Scion of Urza");
    let karn = reg.interner_mut().intern("Karn");
    let _construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(karn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Reveal the top two cards of your library. An opponent \
                       chooses one of them. Put that card into your hand and \
                       exile the other with a silver counter on it.".into(),
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
                text: "-1: Put a card you own with a silver counter on it from \
                       exile into your hand.".into(),
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
                effect: minus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create a 0/0 colorless Construct artifact creature \
                       token with \"This token gets +1/+1 for each artifact you \
                       control.\"".into(),
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
            }),
    )
}

fn plus_one(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: opponent-choice reveal-of-two + silver-counter exile rider.
    Vec::new()
}

fn minus_one(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: retrieve a card from exile filtered by a silver counter — bespoke.
    Vec::new()
}

fn minus_two(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the token's "+1/+1 for each artifact you control" CDA is not
    // expressible; mint a 0/0 colorless Construct artifact creature token.
    let construct = reg.interner().lookup("Construct")
        .expect("Construct interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(construct);
    let token = TokenDefinition {
        name: construct,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
