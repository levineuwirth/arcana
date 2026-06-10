//! Power Armor — `{4}` artifact (Fallen Empires).
//! "Domain — {3}, {T}: Target creature gets +1/+1 until end of turn
//! for each basic land type among lands you control." The domain
//! count is computed at resolution: one for each of the five basic
//! land types represented among lands the activator controls.

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
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Power Armor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "Domain — {3}, {T}: Target creature gets +1/+1 until \
                       end of turn for each basic land type among lands \
                       you control."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: domain_pump,
            },
        ),
    )
}

fn domain_pump(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mut domain: i32 = 0;
    for land_type in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        let filter = script::subtype_filter(reg, land_type)
            .controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &filter, ctx.controller) > 0 {
            domain += 1;
        }
    }
    vec![Effect::Pump {
        target: *id,
        power: domain,
        toughness: domain,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
