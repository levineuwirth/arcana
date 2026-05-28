//! Dwarven Scorcher — `{R}` 1/1 Dwarf.
//! `Sacrifice this creature: This creature deals 1 damage to target creature unless that creature's controller has this creature deal 2 damage to them.`
//! GAP: "unless that creature's controller has this creature deal 2 damage to them" — OptionalPayment can model a life-payment gate, but the "2 damage to that player" framing is a damage-redirection not a simple life-loss; using simplified OptionalPayment(Life(2)) → no-damage, else deal 1 damage to creature.

use arcana_core::actions::OptionalPaymentKind;
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
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwarven Scorcher");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice this creature: This creature deals 1 damage to target creature unless that creature's controller has this creature deal 2 damage to them.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: scorcher_effect,
            }),
    )
}

fn scorcher_effect(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let target_id = *id;
    let controller = script::target_controller(state, target_id, ctx.controller);
    // "unless controller pays 2 life" → OptionalPayment(Life(2)): if they pay, deal 2 to them; else deal 1 to creature
    vec![Effect::OptionalPayment {
        chooser: controller,
        cost: OptionalPaymentKind::Life(2),
        then: Box::new(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(controller),
            amount: 2,
        }),
        else_effect: Some(Box::new(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(target_id),
            amount: 1,
        })),
    }]
}
