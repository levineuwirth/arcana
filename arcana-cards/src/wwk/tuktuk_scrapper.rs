//! Tuktuk Scrapper — `{3}{R}` 2/2 Goblin Artificer Ally.
//! Whenever this creature or another Ally you control enters, you may destroy
//! target artifact. If that artifact is put into a graveyard this way, this
//! creature deals damage to that artifact's controller equal to the number of
//! Allies you control.
//! GAP: The "if that artifact is put into a graveyard this way, deal damage
//! equal to number of Allies to controller" conditional is not expressible.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tuktuk Scrapper");
    let goblin = reg.interner_mut().intern("Goblin");
    let artificer = reg.interner_mut().intern("Artificer");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(artificer);
    subtypes.0.insert(ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // Trigger condition: any Ally you control enters (includes self since self is Ally)
    let ally_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![ally]);
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ally_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: ally_enters_destroy_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn ally_enters_destroy_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        // GAP: "if that artifact is put into a graveyard this way, deal damage
        // to its controller equal to the number of Allies you control" is not
        // expressible (no conditional destroy + on-death damage in catalog).
        Effect::DestroyPermanent { target: *id },
    ]
}
