//! Tezzeret, Master of the Bridge — `{4}{U}{B}` Legendary Planeswalker — Tezzeret.
//! Starting loyalty inferred 5.
//! Static: Creature and planeswalker spells you cast have affinity for
//!   artifacts.
//! +2: Tezzeret deals X damage to each opponent, where X is the number of
//!   artifacts you control. You gain X life.
//! −3: Return target artifact card from your graveyard to your hand.
//! −8: Exile the top ten cards of your library. Put all artifact cards from
//!   among them onto the battlefield.
//!
//! GAP: the "affinity for artifacts" static is a continuous cost reducer, not
//!   a loyalty ability — not modeled here (no loyalty cost).
//! GAP: −3 targets a card in YOUR graveyard — controller-relative graveyard
//!   targeting has no sentinel in the demonstrated surface. Declared with the
//!   correct −3 cost, effect GAP'd.
//! GAP: −8 "exile top ten, put all artifact cards onto the battlefield" — no
//!   exile-top-N-then-put-matching-onto-battlefield primitive in the
//!   demonstrated surface. Declared with the correct −8 cost, effect GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret, Master of the Bridge");
    let tezzeret = reg.interner_mut().intern("Tezzeret");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tezzeret);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}").expect("valid cost")),
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
                text: "+2: Tezzeret deals X damage to each opponent, where X is the \
                       number of artifacts you control. You gain X life.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Return target artifact card from your graveyard to your \
                       hand.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_return,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Exile the top ten cards of your library. Put all artifact \
                       cards from among them onto the battlefield.".into(),
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
                effect: minus_eight,
            }),
    )
}

fn plus_two(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);
    let x = script::count_matching(state, &filter, ctx.controller);
    let mut effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|opp| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(opp),
            amount: x,
        })
        .collect();
    effects.push(Effect::GainLife { player: ctx.controller, amount: x });
    effects
}

fn minus_three_return(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target artifact card from your graveyard" — controller-relative
    // graveyard targeting has no sentinel in the demonstrated surface.
    Vec::new()
}

fn minus_eight(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile-top-N-then-put-matching-onto-battlefield not expressible.
    Vec::new()
}
