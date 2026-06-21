//! The Flood of Mars — `{2}{U}{U}` 3/3 Alien Zombie Horror with
//! Islandwalk. "Water Always Wins — Whenever this creature attacks, put a
//! flood counter on another target creature or land. If it's a creature,
//! it becomes a copy of this creature. If it's a land, it becomes an
//! Island in addition to its other types."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Flood of Mars");
    let alien = reg.interner_mut().intern("Alien");
    let zombie = reg.interner_mut().intern("Zombie");
    let horror = reg.interner_mut().intern("Horror");
    let _flood = reg.interner_mut().intern("flood");
    let _island = reg.interner_mut().intern("Island");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(zombie);
    subtypes.0.insert(horror);

    let island_walk = reg.interner_mut().intern("Island");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(island_walk)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_flood,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::LAND)),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn attack_flood(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let Some(flood) = reg.interner().lookup("flood") else {
        return Vec::new();
    };
    // The flood counter is placed faithfully.
    // GAP: the type-dependent rider ("if it's a creature, it becomes a copy
    // of this creature; if it's a land, it becomes an Island in addition to
    // its other types") branches on the chosen target's CARD TYPE at
    // resolution. Effect::Conditional evaluates a static board predicate,
    // not the per-target type, so the copy/Island transform is omitted.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(flood),
        count: 1,
    }]
}
