//! Vulpine Harvester — `{3}{W}` 3/3 white Phyrexian Fox.
//! "Whenever one or more Phyrexians you control attack, return target artifact card
//! from your graveyard to the battlefield if its mana value is less than or equal
//! to their total power."
//!
//! GAP: "if its mana value is less than or equal to total power of attacking
//! Phyrexians" is a conditional tied to dynamic values not in the Effect catalog.
//! Also, CreatureAttacks trigger fires per-attacker, not "one or more" as a batch.

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

use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vulpine Harvester");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let fox = reg.interner_mut().intern("Fox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(fox);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_subtypes_any(vec![phyrexian]),
                },
                intervening_if: None,
                effect: return_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn return_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: conditional "if its mana value <= total power of attacking Phyrexians"
    // not expressible; returning unconditionally.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
