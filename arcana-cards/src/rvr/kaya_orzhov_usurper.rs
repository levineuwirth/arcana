//! Kaya, Orzhov Usurper — `{1}{W}{B}` Legendary Planeswalker — Kaya,
//! starting loyalty 4.
//!
//! +1: Exile up to two target cards from a single graveyard. You gain 2
//!     life if at least one creature card was exiled this way.
//!     (GAP — graveyard-targeting.)
//! −1: Exile target nonland permanent with mana value 1 or less.
//! −5: Kaya deals damage to target player equal to the number of cards
//!     that player owns in exile and you gain that much life.
//!     (GAP — dynamic amount from exile count.)

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaya, Orzhov Usurper");
    let kaya = reg.interner_mut().intern("Kaya");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaya);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Exile up to two target cards from a single graveyard. You gain 2 life if at least one creature card was exiled this way.".into(),
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
                effect: plus_one_gy_exile,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Exile target nonland permanent with mana value 1 or less.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into())
                            .with_max_cmc(1),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_exile,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-5: Kaya deals damage to target player equal to the number of cards that player owns in exile and you gain that much life.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_exile_drain,
            }),
    )
}

fn plus_one_gy_exile(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Exile up to two target cards from a single graveyard" requires
    //      targeting cards in a graveyard (no any-graveyard sentinel in the
    //      demonstrated surface), plus the conditional life gain keyed to
    //      whether a creature card was exiled. Not expressible.
    Vec::new()
}

fn minus_one_exile(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}

fn minus_five_exile_drain(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: damage + life gain equal to the number of cards the target player
    //      OWNS in exile. There is no exile-zone-by-owner count helper in the
    //      demonstrated surface (script:: counts battlefield/graveyard
    //      objects, not owner-exile), so the dynamic amount can't be derived.
    Vec::new()
}
