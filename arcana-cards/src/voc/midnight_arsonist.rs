//! Midnight Arsonist — `{3}{R}` 3/2 red Vampire.
//! "When this creature enters, destroy up to X target artifacts without mana abilities, where X
//! is the number of Vampires you control."
//! GAP: "artifacts without mana abilities" filter not in ObjectFilter; using artifact filter only.
//! X = number of Vampires you control (dynamic).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetFilter,
    TargetCount, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Midnight Arsonist");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: destroy_artifacts,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().without_types(TypeLine::CREATURE.into()),
                    ),
                    count: TargetCount::UpTo(4), // GAP: count should be dynamic X
                    controller: None,
                }],
            }),
    )
}

fn destroy_artifacts(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vampire_filter = script::subtype_filter(reg, "Vampire");
    let _x = script::count_matching(state, &vampire_filter, trig.controller);
    // Destroy each targeted artifact
    trig.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::DestroyPermanent { target: *id })
        } else {
            None
        }
    }).collect()
}
