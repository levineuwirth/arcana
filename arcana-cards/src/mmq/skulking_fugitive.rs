//! Skulking Fugitive — `{2}{B}` 3/4 Horror Mercenary. "When this
//! creature becomes the target of a spell or ability, sacrifice it."

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
    let name = reg.interner_mut().intern("Skulking Fugitive");
    let horror = reg.interner_mut().intern("Horror");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    subtypes.0.insert(mercenary);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: on_becomes_target_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_becomes_target_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::CREATURE.into())
            .controlled_by(ControllerConstraint::You),
        count: 1,
    }]
}
