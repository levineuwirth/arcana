//! Karn, Living Legacy — `{4}` Legendary Planeswalker — Karn,
//! starting loyalty 5. Colorless.
//!
//! +1: Create a tapped Powerstone token. (Modeled via CreateCommodityToken;
//!     the "tapped" rider isn't expressible on that variant — the token enters
//!     untapped.)
//! −1: Pay any amount of mana. Look at that many cards from the top of your
//!     library, then put one into your hand and the rest on the bottom in a
//!     random order. GAP: "pay any amount of mana" is a dynamic-X cost/dig with
//!     no demonstrated primitive.
//! −7: You get an emblem with "Tap an untapped artifact you control: This
//!     emblem deals 1 damage to any target." GAP: emblem creation with a
//!     bespoke activated ability is not expressible.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karn, Living Legacy");
    let karn = reg.interner_mut().intern("Karn");
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
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Pay any amount of mana. Look at that many cards from \
                       the top of your library, then put one of those cards into \
                       your hand and the rest on the bottom of your library in a \
                       random order.".into(),
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
                text: "-7: You get an emblem with \"Tap an untapped artifact you \
                       control: This emblem deals 1 damage to any target.\"".into(),
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
                effect: minus_seven,
            }),
    )
}

fn plus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "tapped" rider not expressible on CreateCommodityToken; mint a Powerstone.
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Powerstone,
        count: 1,
    }]
}

fn minus_one(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "pay any amount of mana" is a dynamic-X cost feeding a variable dig.
    Vec::new()
}

fn minus_seven(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a bespoke activated ability.
    Vec::new()
}
