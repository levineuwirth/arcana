//! Crimson Hellkite — `{6}{R}{R}{R}` 6/6 red Dragon with Flying.
//!
//! * Flying.
//! * "{X}, {T}: This creature deals X damage to target creature. Spend
//!   only red mana on X." — implemented as an `{X}`+tap activation that
//!   deals `ctx.x_value` damage to the targeted creature. FIDELITY GAP:
//!   "Spend only red mana on X" is a payment restriction with no cost
//!   field to express it (X is paid as generic mana).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crimson Hellkite");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{X}, {T}: This creature deals X damage to target creature."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}, {T}: Crimson Hellkite deals X damage to target \
                       creature. Spend only red mana on X."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_x_damage,
            }),
    )
}

fn deal_x_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let amount = ctx.x_value.unwrap_or(0);
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
