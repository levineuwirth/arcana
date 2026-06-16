//! Reaper King — `{2/W}{2/U}{2/B}{2/R}{2/G}` 6/6 Legendary Artifact Creature — Scarecrow.
//! Other Scarecrow creatures you control get +1/+1.
//! Whenever another Scarecrow you control enters, destroy target permanent.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
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
    let name = reg.interner_mut().intern("Reaper King");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2/W}{2/U}{2/B}{2/R}{2/G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    let scarecrow_filter =
        script::subtype_filter(reg, "Scarecrow").controlled_by(ControllerConstraint::You);
    // GAP static: "Other Scarecrow creatures you control get +1/+1" — a
    // continuous anthem, not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // NOTE: "another" Scarecrow exclusion is not expressible in the
                // ZoneChange filter; self-entry is the only over-fire case.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: scarecrow_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: destroy_target_permanent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn destroy_target_permanent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}
