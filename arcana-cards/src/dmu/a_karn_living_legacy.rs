//! A-Karn, Living Legacy — `{4}` Legendary Planeswalker — Karn, starting loyalty 5.
//!
//! +1: Create a tapped Powerstone token. Modeled via
//!   `Effect::CreateCommodityToken { Powerstone }` (the "enters tapped"
//!   rider is a minor fidelity gap — the commodity-token builder has no
//!   tapped flag).
//! 0: Pay any amount of mana, look at that many cards, keep one, bottom
//!   the rest. GAP: dynamic-X "pay any amount" cost + look-keep-bottom
//!   choice is not expressible from the demonstrated surface.
//! −6: You get an emblem with "Tap an untapped artifact you control:
//!   This emblem deals 1 damage to any target." The emblem's grant is an
//!   ACTIVATED ability (a tap-an-artifact cost), and the emblem
//!   `abilities` slot only models TRIGGERED abilities — so the emblem is
//!   created but its granted ability is GAP'd (no static/triggered form).

use arcana_core::effects::{CommodityToken, Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Karn, Living Legacy");
    let karn = reg.interner_mut().intern("Karn");
    let _emblem = reg.interner_mut().intern("A-Karn, Living Legacy emblem");
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
                text: "+1: Create a tapped Powerstone token.".into(),
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
                effect: plus_one_powerstone,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Pay any amount of mana. Look at that many cards \
                       from the top of your library, then put one of those \
                       cards into your hand and the rest on the bottom of \
                       your library in a random order.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"Tap an untapped artifact \
                       you control: This emblem deals 1 damage to any \
                       target.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

fn plus_one_powerstone(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Powerstone,
        count: 1,
    }]
}

fn zero_dig(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "pay any amount of mana" is a dynamic-X cost and the
    // look-many/keep-one/bottom-rest choice is not expressible from the
    // demonstrated Effect surface.
    Vec::new()
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("A-Karn, Living Legacy emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // GAP: the emblem's grant is an ACTIVATED ability ("Tap an
            // untapped artifact you control: deal 1 damage"). The emblem
            // `abilities` slot models triggered abilities only, so the
            // granted ability can't be expressed.
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
