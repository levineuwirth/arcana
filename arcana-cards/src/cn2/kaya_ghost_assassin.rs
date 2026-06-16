//! Kaya, Ghost Assassin — `{2}{W}{B}` Legendary Planeswalker — Kaya,
//! starting loyalty 5.
//!
//! All three loyalty abilities are GAP'd at the effect level (shells with
//! correct loyalty costs are still emitted):
//!
//! * `0`: Exile Kaya or up to one target creature and return it at the
//!   beginning of your next upkeep; you lose 2 life. Needs a delayed
//!   exile-and-return-at-next-upkeep bundle — not expressible here.
//! * `−1`: Each opponent loses 2 life and you gain 2 life. "Each opponent
//!   loses life" has no all-opponents life-loss variant in the
//!   demonstrated surface (`Effect::LoseLife` is single-player).
//! * `−2`: Each opponent discards a card and you draw a card. Same
//!   all-opponents limitation for the discard.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaya, Ghost Assassin");
    let kaya = reg.interner_mut().intern("Kaya");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaya);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Exile Kaya or up to one target creature. Return that \
                       card to the battlefield under its owner's control at the \
                       beginning of your next upkeep. You lose 2 life.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_exile_return,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Each opponent loses 2 life and you gain 2 life.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_drain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Each opponent discards a card and you draw a card.".into(),
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
                effect: minus_two_discard_draw,
            }),
    )
}

fn zero_exile_return(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile (self or target) then return-at-next-upkeep delayed
    // bundle is not expressible from the demonstrated surface.
    Vec::new()
}

fn minus_one_drain(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each opponent loses life" has no all-opponents variant.
    Vec::new()
}

fn minus_two_discard_draw(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each opponent discards a card" has no all-opponents variant.
    Vec::new()
}
