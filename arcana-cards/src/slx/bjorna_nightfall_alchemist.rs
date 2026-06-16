//! Bjorna, Nightfall Alchemist — `{U}{R}` 1/3 Legendary Human.
//! {T}, Sacrifice an artifact: Bjorna deals 1 damage to target creature. Goad
//! that creature.
//! Partner—Friends forever. (GAP — not in the supported KeywordAbility set.)

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bjorna, Nightfall Alchemist");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice an artifact: Bjorna, Nightfall Alchemist deals 1 damage to target creature. Goad that creature.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice_other: Some(
                    ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
                ),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: ping_and_goad,
        }),
    )
}

fn ping_and_goad(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(*id),
            amount: 1,
        },
        Effect::Goad {
            target: *id,
            goader: ctx.controller,
            duration: Duration::UntilYourNextTurn(ctx.controller),
        },
    ]
}
