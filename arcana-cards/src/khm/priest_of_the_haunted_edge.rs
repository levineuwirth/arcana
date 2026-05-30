//! Priest of the Haunted Edge — `{1}{B}` 0/4 black Snow Creature — Zombie Cleric.
//! `{T}, Sacrifice this creature: Target creature gets -X/-X until end
//! of turn, where X is the number of snow lands you control. Activate
//! only as a sorcery.`

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Priest of the Haunted Edge");
    let zombie = reg.interner_mut().intern("Zombie");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice this creature: Target creature gets -X/-X until end of turn, where X is the number of snow lands you control. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_x_minus_x,
            }),
    )
}

fn minus_x_minus_x(
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
    // X = number of snow lands you control
    let snow_land_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You)
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::SNOW));
    let x = script::count_matching(state, &snow_land_filter, ctx.controller) as i32;
    vec![Effect::Pump {
        target: *id,
        power: -x,
        toughness: -x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
