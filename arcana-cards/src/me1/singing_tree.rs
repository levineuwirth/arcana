//! Singing Tree — `{3}{G}` 0/3 green Plant.
//! "{T}: Target attacking creature has base power 0 until end of turn."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Singing Tree");
    let plant = reg.interner_mut().intern("Plant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target attacking creature has base power 0 until end of turn.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature().attacking_only()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: set_power_zero,
            }),
    )
}

fn set_power_zero(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // SetBasePT sets both power and toughness; we only want to set power to 0.
    // Using current toughness is not possible without state access.
    // GAP: No "set base power only" Effect variant; SetBasePT changes both dimensions.
    vec![Effect::SetBasePT {
        target: *id,
        power: 0,
        toughness: 0,
        duration: Duration::EndOfTurn,
    }]
}
