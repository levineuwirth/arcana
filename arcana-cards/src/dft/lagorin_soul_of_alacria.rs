//! Lagorin, Soul of Alacria — `{G}{W}` 1/1 Legendary Beast Mount with Flying.
//! "Whenever Lagorin attacks while saddled, put a +1/+1 counter on each of up to
//!  two target Mounts and/or Vehicles.
//!  Saddle 1 (…)"

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lagorin, Soul of Alacria");
    let beast = reg.interner_mut().intern("Beast");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    subtypes.0.insert(mount);

    let mount_sym = reg.interner_mut().intern("Mount");
    let vehicle_sym = reg.interner_mut().intern("Vehicle");
    let mount_or_vehicle = ObjectFilter::permanent()
        .with_subtypes_any(vec![mount_sym, vehicle_sym]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Saddle 1" — Saddle is not a supported KeywordAbility variant and its
    // tap-creatures-totaling-power cost is not an expressible ActivationCost.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP rider: "while saddled" gate is not expressible (no saddled
                // intervening-if); the trigger fires on every attack instead.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: counter_mounts,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(mount_or_vehicle),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            }),
    )
}

fn counter_mounts(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
            _ => None,
        })
        .collect()
}
