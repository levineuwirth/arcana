//! Jaya Ballard — `{2}{R}{R}{R}` Legendary Planeswalker — Jaya.
//! Starting loyalty 5 (oracle).
//!
//! +1: Add {R}{R}{R}. Spend this mana only to cast instant or sorcery spells.
//!     PARTIAL: the three red mana are added. GAP: the "spend only on
//!     instant/sorcery" restriction is not attached (no mana-restriction
//!     surface on Effect::AddMana for generated cards here).
//! +1: Discard up to three cards, then draw that many cards.
//!     GAP: "discard up to three, then draw THAT MANY" — a variable discard
//!     count feeding a dynamic draw count is not expressible (Effect::Discard
//!     takes a fixed count and there's no draw-equal-to-discarded primitive).
//! −8: You get an emblem. GAP: emblem creation not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet,
    TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jaya Ballard");
    let jaya = reg.interner_mut().intern("Jaya");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jaya);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R}{R}{R}. Spend this mana only to cast instant \
                       or sorcery spells.".into(),
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
                effect: plus_one_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Discard up to three cards, then draw that many \
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
                effect: plus_one_loot_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn plus_one_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // PARTIAL: add {R}{R}{R}. GAP: the instant/sorcery-only spend restriction.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
        ],
    }]
}

fn plus_one_loot_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard up to three cards, then draw that many" — variable discard
    //      feeding a dynamic draw not expressible.
    Vec::new()
}

fn minus_eight_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation not modeled.
    Vec::new()
}
