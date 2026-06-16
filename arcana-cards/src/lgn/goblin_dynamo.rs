//! Goblin Dynamo — `{5}{R}{R}` Creature — Goblin Mutant, 4/4.
//!
//! Oracle:
//! * `{T}: This creature deals 1 damage to any target.`
//! * `{X}{R}, {T}, Sacrifice this creature: It deals X damage to any target.`
//!
//! The first activated ability is a canonical pinger and is fully expressed.
//! The second's damage scales with `{X}` from the activation mana cost; the
//! `ActivateAbility` action does not thread a generic-`{X}` choice into
//! `ActivationContext::x_value` (only loyalty-X does), so the X-scaled damage
//! is GAP'd while the ability (cost `{R}`, tap, sacrifice self) and its target
//! are still emitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Dynamo");
    let goblin = reg.interner_mut().intern("Goblin");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: This creature deals 1 damage to any target.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{R}, {T}, Sacrifice this creature: It deals X damage to any target.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_x_damage,
            }),
    )
}

fn ping_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 1,
    }]
}

fn deal_x_damage(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "It deals X damage to any target", where X is the {X} paid in the
    // activation mana cost. The ActivateAbility action does not thread a
    // generic-{X} choice into ActivationContext::x_value (only loyalty-X does),
    // so the damage amount cannot be read here. The cost ({R}, tap, sacrifice)
    // and the any-target requirement are still emitted above.
    Vec::new()
}
