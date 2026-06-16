//! A-Tatyova, Steward of Tides — `{G}{G}{U}` 4/4 Legendary Merfolk Druid.
//! "Land creatures you control have flying." (static — GAP'd below.)
//! "Whenever a land enters under your control, if you control seven or
//! more lands, up to one target land you control becomes a 4/4 Elemental
//! creature with haste. It's still a land."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, PlayerId, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::conditions;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Tatyova, Steward of Tides");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Land creatures you control have flying" is a continuous
    // type-filtered keyword grant with no trigger/cost — not expressible here.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::new()
                    .with_types(TypeLine::LAND.into())
                    .controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: Some(if_seven_lands),
            effect: animate_land,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn if_seven_lands(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::you_control_at_least(
        s,
        you,
        &ObjectFilter::new().with_types(TypeLine::LAND.into()),
        7,
    )
}

fn animate_land(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // Becomes a 4/4 Elemental creature with haste that's still a land.
    // GAP: granting the Elemental creature subtype is not expressible.
    vec![
        Effect::AddType {
            target: *id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::Permanent,
        },
        Effect::SetBasePT {
            target: *id,
            power: 4,
            toughness: 4,
            duration: Duration::Permanent,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::Permanent,
        },
    ]
}
