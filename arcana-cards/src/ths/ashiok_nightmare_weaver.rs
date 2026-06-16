//! Ashiok, Nightmare Weaver — `{1}{U}{B}` Legendary Planeswalker — Ashiok.
//! Starting loyalty inferred 3.
//! +2: Exile the top three cards of target opponent's library.
//! −X: Put a creature card with mana value X exiled with Ashiok onto the
//!   battlefield under your control. That creature is a Nightmare in addition
//!   to its other types.
//! −10: Exile all cards from all opponents' hands and graveyards.
//!
//! GAP: +2 "exile the top three cards of target opponent's library" — no
//!   exile-top-N-of-a-target-player's-library primitive in the demonstrated
//!   surface (Mill goes to graveyard; ImpulseExile is own-library). Declared
//!   with the correct +2 cost and a target-opponent requirement, effect GAP'd.
//! GAP: −X is a dynamic chosen loyalty cost; `remove_self_counter` is a fixed
//!   u32, so dynamic-X loyalty costs are not expressible — ability OMITTED.
//! GAP: −10 — opponents' hands cannot be exiled (no exile-from-hand effect in
//!   the demonstrated surface). The opponents'-graveyard portion is modeled;
//!   the hand portion is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ashiok, Nightmare Weaver");
    let ashiok = reg.interner_mut().intern("Ashiok");
    let _nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ashiok);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Exile the top three cards of target opponent's library.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_exile_library,
            })
            // −X ability OMITTED: dynamic-X loyalty cost is not expressible.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: Exile all cards from all opponents' hands and graveyards.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten,
            }),
    )
}

fn plus_two_exile_library(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no exile-top-N-of-a-target-player's-library primitive.
    Vec::new()
}

fn minus_ten(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: opponents' hands cannot be exiled (no exile-from-hand effect).
    // The opponents'-graveyard portion is modeled.
    let mut effects = Vec::new();
    for opp in script::opponents(state, ctx.controller) {
        let ids: Vec<_> = state
            .objects
            .objects_in_zone(Zone::Graveyard(opp))
            .map(|o| o.id)
            .collect();
        for id in ids {
            effects.push(Effect::ExileFromGraveyard { target: id });
        }
    }
    effects
}
