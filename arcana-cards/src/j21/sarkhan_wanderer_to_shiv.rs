//! Sarkhan, Wanderer to Shiv — `{3}{R}` Legendary Planeswalker — Sarkhan,
//! starting loyalty 5.
//!
//! * `+1`: Dragon cards in your hand perpetually gain cost-reduction and
//!   alternative-cost riders. GAP'd (perpetual / "you may pay {X}" riders
//!   are not expressible in the demonstrated surface).
//! * `+1`: Conjure a card named Shivan Dragon into your hand. GAP'd (no
//!   Conjure effect in the demonstrated surface).
//! * `−2`: Sarkhan deals 3 damage to target creature. `Effect::DealDamage`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarkhan, Wanderer to Shiv");
    let sarkhan = reg.interner_mut().intern("Sarkhan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sarkhan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
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
                text: "+1: Dragon cards in your hand perpetually gain \"This \
                       spell costs {1} less to cast,\" and \"You may pay {X} \
                       rather than pay this spell's mana cost, where X is its \
                       mana value.\"".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_perpetual,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Conjure a card named Shivan Dragon into your hand.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_conjure,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Sarkhan, Wanderer to Shiv deals 3 damage to target \
                       creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_bolt,
            }),
    )
}

fn plus_one_perpetual(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: perpetual cost-reduction / alternative-cost riders on hand cards
    // are not expressible in the demonstrated surface.
    Vec::new()
}

fn plus_one_conjure(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure (create a named card into hand) is not in the
    // demonstrated Effect surface.
    Vec::new()
}

fn minus_two_bolt(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 3,
    }]
}
