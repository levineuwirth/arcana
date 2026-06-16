//! Liliana, the Last Hope — `{1}{B}{B}` Legendary Planeswalker — Liliana,
//! starting loyalty 3.
//!
//! +1: Up to one target creature gets -2/-1 until your next turn.
//! −2: Mill two cards, then you may return a creature card from your graveyard
//!   to your hand.
//! −7: You get an emblem with "At the beginning of your end step, create X 2/2
//!   black Zombie creature tokens, where X is two plus the number of Zombies
//!   you control."
//!
//! GAP: the −2's "return a creature card from your graveyard to your hand"
//!   needs a concrete graveyard target; only the "mill two cards" half is
//!   emitted.
//! GAP: the −7 ultimate creates an emblem — emblem creation is not in the
//!   demonstrated Effect surface; declared with its −7 cost, effect empty.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana, the Last Hope");
    let liliana = reg.interner_mut().intern("Liliana");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(liliana);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature gets -2/-1 until your next \
                       turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_shrink,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Mill two cards, then you may return a creature card \
                       from your graveyard to your hand.".into(),
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
                effect: minus_two_mill,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"At the beginning of your end \
                       step, create X 2/2 black Zombie creature tokens, where X \
                       is two plus the number of Zombies you control.\"".into(),
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
                effect: minus_seven_emblem,
            }),
    )
}

/// `+1: Up to one target creature gets -2/-1 until your next turn.`
fn plus_one_shrink(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -2,
        toughness: -1,
        duration: Duration::UntilYourNextTurn(ctx.controller),
        keywords: vec![],
    }]
}

/// `−2: Mill two cards, then you may return a creature card …`
fn minus_two_mill(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return a creature card from your graveyard to your hand" needs a
    // concrete graveyard target; only the mill is emitted.
    vec![Effect::Mill { player: ctx.controller, count: 2 }]
}

/// `−7: You get an emblem …`
fn minus_seven_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation is not expressible with the demonstrated Effect
    // surface.
    Vec::new()
}
