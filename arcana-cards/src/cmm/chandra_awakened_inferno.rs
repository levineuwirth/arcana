//! Chandra, Awakened Inferno — `{4}{R}{R}` Legendary Planeswalker — Chandra,
//! starting loyalty 6.
//!
//! "This spell can't be countered." — a static cast-time characteristic, not a
//!   loyalty ability and not expressible from the demonstrated surface. GAP
//!   (static, not modeled).
//! +2: Each opponent gets an emblem with "At the beginning of your upkeep, this
//!   emblem deals 1 damage to you." GAP — `Effect::CreateEmblem` has a single
//!   `controller` field (the PW's controller); granting an emblem to EACH
//!   OPPONENT is not expressible. Correct +2 cost retained, effect GAP'd.
//! −3: Chandra deals 3 damage to each non-Elemental creature. Modeled via a
//!   resolution-time sweep of non-Elemental creatures.
//! −X: Chandra deals X damage to target creature or planeswalker (exile if it
//!   would die). GAP — dynamic-X loyalty cost is not expressible; ability omitted.

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
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Awakened Inferno");
    let chandra = reg.interner_mut().intern("Chandra");
    let _elemental = reg.interner_mut().intern("Elemental");
    let _emblem = reg.interner_mut().intern("Chandra, Awakened Inferno emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Each opponent gets an emblem with \"At the beginning \
                       of your upkeep, this emblem deals 1 damage to you.\"".into(),
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
                effect: plus_two_opponent_emblem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Chandra, Awakened Inferno deals 3 damage to each \
                       non-Elemental creature.".into(),
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
                effect: minus_three_sweep,
            }),
    )
}

fn plus_two_opponent_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Each opponent gets an emblem ..." — CreateEmblem's single
    //      `controller` field cannot grant an emblem to each opponent; the
    //      self-damage trigger reads "you" = the emblem's controller (an
    //      opponent), which is not addressable here. Emblem shell omitted.
    let _ = reg.interner().lookup("Chandra, Awakened Inferno emblem");
    let _ = ctx.controller;
    Vec::new()
}

fn minus_three_sweep(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental").expect("Elemental interned at register");
    let filter = ObjectFilter::creature().without_subtype_sym(elemental);
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 3,
        })
        .collect()
}
