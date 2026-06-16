//! Nahiri, Storm of Stone — `{2}{R/W}{R/W}` Legendary Planeswalker — Nahiri, starting loyalty 5.
//!
//! Static (NOT a loyalty ability): During your turn, creatures you
//!   control have first strike and equip abilities you activate cost {1}
//!   less to activate. GAP: a turn-gated controller-wide keyword anthem
//!   plus an equip-cost reduction; the demonstrated builders for an
//!   on-card static can't express either (no "during your turn" duration
//!   gate, no cost-reduction static). Noted, not modeled.
//! −X: Nahiri deals X damage to target tapped creature. IMPLEMENTED via
//!   dynamic-X loyalty (remove_loyalty_x) dealing X = loyalty paid to a
//!   tapped-creature target.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nahiri, Storm of Stone");
    let nahiri = reg.interner_mut().intern("Nahiri");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nahiri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/W}{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "−X: Nahiri deals X damage to target tapped creature.".into(),
            cost: ActivationCost {
                remove_loyalty_x: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature().tapped_only()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: true,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: minus_x_damage,
        }),
    )
}

/// `−X` — deal X damage to target tapped creature.
fn minus_x_damage(
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
        amount: ctx.x_value.unwrap_or(0),
    }]
}
