//! Mr. Monopoly, On the Go — `{3}{R}` legendary planeswalker, starting loyalty 4.
//!
//! 0: Roll a six-sided die. Put a number of loyalty counters on Mr.
//!    Monopoly equal to the result (die roll + dynamic loyalty add — GAP).
//! −2: Heist! — Exile top two cards of target opponent's library; you may
//!    play them this turn, any mana works (opponent-impulse rider — GAP).
//! −4: Shut Down! — Destroy target artifact.
//! −40: Pass Go — Create 200 Treasure tokens.
//!
//! Scope: the −4 (destroy artifact) and −40 (create 200 Treasure) are
//! fully expressed. The 0 die-roll-to-loyalty and the −2
//! opponent-library impulse with a play-this-turn rider have no
//! demonstrated Effect and are GAP'd (shells declared with correct cost).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Mr. Monopoly, On the Go");
    let monopoly = reg.interner_mut().intern("Monopoly");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(monopoly);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    let artifact = ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into());

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Roll a six-sided die. Put a number of loyalty counters \
                       on Mr. Monopoly equal to the result.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_roll,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Heist! — Exile the top two cards of target opponent's \
                       library. Until end of turn, you may play those cards, and \
                       mana of any type can be spent to cast them.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_heist,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−4: Shut Down! — Destroy target artifact.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(artifact),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−40: Pass Go — Create 200 Treasure tokens.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 40)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_forty_treasure,
            }),
    )
}

/// `0` — die roll converting to loyalty.
fn zero_roll(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "roll a six-sided die, add that many loyalty counters" — no
    // die-roll Effect, and the loyalty add is dynamic-X.
    Vec::new()
}

/// `−2` — opponent-library impulse with play rider.
fn minus_two_heist(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: exile from a target OPPONENT's library + "you may play them this
    // turn, any mana" rider not expressible (ImpulseExile is own-library).
    Vec::new()
}

/// `−4: Destroy target artifact.`
fn minus_four_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `−40: Create 200 Treasure tokens.`
fn minus_forty_treasure(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Treasure,
        count: 200,
    }]
}
