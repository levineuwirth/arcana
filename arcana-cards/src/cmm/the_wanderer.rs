//! The Wanderer — `{3}{W}` legendary planeswalker, starting loyalty 3.
//!
//! Static: Prevent all noncombat damage that would be dealt to you and
//!   other permanents you control (static prevention, no loyalty cost —
//!   not a loyalty ability — GAP).
//! −2: Exile target creature with power 4 or greater.
//!
//! Scope: the −2 ability is fully expressed (exile a power-4-or-greater
//! creature). The static noncombat-damage prevention has no loyalty cost
//! and no demonstrated targeted-static-prevention Effect — noted GAP.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("The Wanderer");
    let wanderer = reg.interner_mut().intern("Wanderer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wanderer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    // GAP: "Prevent all noncombat damage to you and your permanents" static
    // ability has no loyalty cost and no demonstrated prevention Effect.

    let power_four = ObjectFilter::creature().with_min_power(4);

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Exile target creature with power 4 or greater.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(power_four),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_exile,
            }),
    )
}

/// `−2: Exile target creature with power 4 or greater.`
fn minus_two_exile(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}
