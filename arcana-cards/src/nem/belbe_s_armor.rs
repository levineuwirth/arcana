//! Belbe's Armor — `{3}` artifact (Nemesis, 2000).
//! "{X}, {T}: Target creature gets -X/+X until end of turn." The X is
//! read from the activation's x_value and applied as a -X/+X pump.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Belbe's Armor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{X}, {T}: Target creature gets -X/+X until end of \
                       turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: shift_pt,
            },
        ),
    )
}

fn shift_pt(
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
    let x = ctx.x_value.unwrap_or(0) as i32;
    vec![Effect::Pump {
        target: *id,
        power: -x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
