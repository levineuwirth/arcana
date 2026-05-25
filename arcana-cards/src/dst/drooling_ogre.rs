//! Drooling Ogre — `{1}{R}` 3/3 Ogre.
//! "Whenever a player casts an artifact spell, that player gains
//! control of this creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drooling Ogre");
    let ogre = reg.interner_mut().intern("Ogre");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types_any: Some(TypeLine::ARTIFACT.into()),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: on_artifact_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_artifact_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(caster) = trig.triggering_caster() else {
        return Vec::new();
    };
    vec![Effect::ChangeControl {
        target: trig.source,
        new_controller: caster,
    }]
}
