//! Kor Outfitter — `{W}{W}` 2/2 white Kor Soldier.
//! "When this creature enters, you may attach target Equipment you
//! control to target creature you control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kor Outfitter");
    let kor = reg.interner_mut().intern("Kor");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_attach,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // Two targets: equipment and creature
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent()
                                .with_types(TypeLine::ARTIFACT.into())
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn etb_attach(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(equipment_target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(equipment_id) = equipment_target else { return Vec::new(); };
    let Some(creature_target) = trig.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(creature_id) = creature_target else { return Vec::new(); };
    vec![Effect::Attach {
        equipment_or_aura: *equipment_id,
        target: *creature_id,
    }]
}
