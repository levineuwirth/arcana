//! Ashiok, Nightmare Muse — `{3}{U}{B}` Legendary Planeswalker — Ashiok,
//! starting loyalty 5.
//!
//! * `+1`: Create a 2/3 blue and black Nightmare creature token with an
//!   attack/block exile-mill trigger. The token is minted; its triggered
//!   ability ("each opponent exiles the top two cards of their library")
//!   is GAP'd (all-opponents library exile not expressible).
//! * `−3`: Return target nonland permanent to its owner's hand
//!   (`Effect::ReturnToHand`); the "then that player exiles a card from
//!   their hand" rider is GAP'd.
//! * `−7`: Cast up to three spells from opponents' exiled face-up cards
//!   for free. GAP'd (no demonstrated surface).

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
    let name = reg.interner_mut().intern("Ashiok, Nightmare Muse");
    let ashiok = reg.interner_mut().intern("Ashiok");
    let _nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ashiok);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 2/3 blue and black Nightmare creature token \
                       with \"Whenever this token attacks or blocks, each \
                       opponent exiles the top two cards of their library.\"".into(),
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
                effect: plus_one_nightmare,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Return target nonland permanent to its owner's hand, \
                       then that player exiles a card from their hand.".into(),
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
                effect: minus_three_bounce,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You may cast up to three spells from among face-up \
                       cards your opponents own from exile without paying their \
                       mana costs.".into(),
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
                effect: minus_seven_cast,
            }),
    )
}

fn plus_one_nightmare(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nightmare = reg.interner().lookup("Nightmare")
        .expect("Nightmare interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    // GAP: the token's "attacks or blocks → each opponent exiles top two"
    // trigger is not expressible (all-opponents library exile). Minted
    // as a vanilla 2/3.
    let token = TokenDefinition {
        name: nightmare,
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_three_bounce(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: the "then that player exiles a card from their hand" rider is
    // not expressible; the bounce is.
    vec![Effect::ReturnToHand { target: *id }]
}

fn minus_seven_cast(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cast up to three spells from opponents' exiled cards for free.
    Vec::new()
}
