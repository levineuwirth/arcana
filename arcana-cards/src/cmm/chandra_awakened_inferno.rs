//! Chandra, Awakened Inferno — `{4}{R}{R}` Legendary Planeswalker —
//! Chandra, starting loyalty 6.
//!
//! "This spell can't be countered" is a cast-time static property, not a
//! loyalty ability; not modeled here.
//!
//! * `+2`: Each opponent gets an emblem dealing 1 damage to them each
//!   upkeep. GAP'd (emblem creation for opponents).
//! * `−3`: Chandra deals 3 damage to each non-Elemental creature.
//!   Resolution-time `script::ids_matching` (creatures without the
//!   Elemental subtype) + per-id `DealDamage`.
//! * `−X`: Chandra deals X damage to target creature or planeswalker;
//!   exile-instead-of-die rider. OMITTED — dynamic-X loyalty cost is not
//!   expressible (`remove_self_counter` is a fixed `u32`).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Awakened Inferno");
    let chandra = reg.interner_mut().intern("Chandra");
    let _elemental = reg.interner_mut().intern("Elemental");
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
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_emblem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Chandra deals 3 damage to each non-Elemental \
                       creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_sweep,
            }),
            // GAP: "−X: Chandra deals X damage … exile instead" — dynamic-X
            // loyalty cost is not expressible; the ability is omitted.
    )
}

fn plus_two_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: each opponent gets an upkeep-damage emblem.
    Vec::new()
}

fn minus_three_sweep(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental")
        .expect("Elemental interned during register()");
    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().without_subtype_sym(elemental),
        ctx.controller,
    );
    creatures
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 3,
        })
        .collect()
}
