//! Kaya, Bane of the Dead — `{3}{W/B}{W/B}{W/B}` Legendary Planeswalker — Kaya, starting loyalty 5.
//!
//! Static: "Your opponents and permanents your opponents control with hexproof
//!   can be the targets of spells and abilities you control as though they didn't
//!   have hexproof." — a targeting-permission static; not a loyalty ability and not
//!   expressible from the demonstrated surface. GAP (static, not modeled).
//! −3: Exile target creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaya, Bane of the Dead");
    let kaya = reg.interner_mut().intern("Kaya");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaya);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W/B}{W/B}{W/B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Exile target creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_exile,
            }),
    )
}

fn minus_three_exile(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::ExilePermanent { target: *id }]
}
