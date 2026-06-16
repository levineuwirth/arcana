//! Liliana, the Necromancer — `{3}{B}{B}` Legendary Planeswalker — Liliana.
//! Color B. Starting loyalty 5.
//!
//! +1: Target player loses 2 life.
//! −1: Return target creature card from your graveyard to your hand.
//! −7: Destroy up to two target creatures. Put up to two creature cards from
//!   graveyards onto the battlefield under your control.
//!
//! GAP: the −7 second clause ("put up to two creature cards from graveyards
//!   onto the battlefield") chooses cards from graveyards as a non-targeted
//!   resolution-time selection across all graveyards; not expressible from the
//!   demonstrated surface. The destroy clause is emitted; the reanimation
//!   clause is omitted from the effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana, the Necromancer");
    let liliana = reg.interner_mut().intern("Liliana");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(liliana);

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
            // +1: Target player loses 2 life.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Target player loses 2 life.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_lose_life,
            })
            // −1: Return target creature card from your graveyard to your hand.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Return target creature card from your graveyard to your hand.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types_any(TypeLine::CREATURE.into()),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_return,
            })
            // −7: Destroy up to two target creatures + reanimate up to two (GAP'd second clause).
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Destroy up to two target creatures. Put up to two creature cards from graveyards onto the battlefield under your control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine::CREATURE.into()),
                    ),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_destroy,
            }),
    )
}

/// `+1: Target player loses 2 life.`
fn plus_one_lose_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::LoseLife { player: *p, amount: 2 }]
}

/// `-1: Return target creature card from your graveyard to your hand.`
fn minus_one_return(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}

/// `-7: Destroy up to two target creatures. (reanimation clause GAP'd)`
fn minus_seven_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for target in &ctx.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::DestroyPermanent { target: *id });
        }
    }
    // GAP: "put up to two creature cards from graveyards onto the battlefield"
    // is a non-targeted resolution-time selection across all graveyards, not
    // expressible from the demonstrated surface.
    effects
}
