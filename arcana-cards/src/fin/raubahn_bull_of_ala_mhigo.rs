//! Raubahn, Bull of Ala Mhigo — `{1}{R}` 2/2 Legendary Human Warrior.
//! "Ward—Pay life equal to Raubahn's power." (GAP — non-mana, dynamic ward cost
//!  is not expressible; KeywordAbility::Ward takes a fixed ManaCost only.)
//! "Whenever Raubahn attacks, attach up to one target Equipment you control to
//!  target attacking creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raubahn, Bull of Ala Mhigo");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: keyword — "Ward—Pay life equal to Raubahn's power." Non-mana,
        // dynamic ward cost; KeywordAbility::Ward only carries a fixed ManaCost.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attach_equipment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // [0] = up to one Equipment you control; [1] = target attacking creature.
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new()
                                .with_types(TypeLine::ARTIFACT.into())
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn attach_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // First target may be absent (up to one Equipment); the attacking creature
    // is the last Object target chosen.
    let objs: Vec<_> = trig
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(*id),
            _ => None,
        })
        .collect();
    // Need both an Equipment and the attacking creature to attach.
    if objs.len() < 2 {
        return Vec::new();
    }
    let equipment_id = objs[0];
    let creature_id = objs[1];
    vec![Effect::Attach {
        equipment_or_aura: equipment_id,
        target: creature_id,
    }]
}
