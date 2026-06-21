//! Omo, Queen of Vesuva — `{2}{G/U}` 1/5 Legendary Shapeshifter Noble.
//! Whenever Omo enters or attacks, put an everything counter on each of up to
//! one target land and up to one target creature.
//! Each land with an everything counter is every land type; each nonland
//! creature with one is every creature type (static type-granting — GAP).

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Omo, Queen of Vesuva");
    let _everything = reg.interner_mut().intern("everything");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // up to one target land, and up to one target creature.
    let reqs = || {
        vec![
            TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            },
            TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::UpTo(1),
                controller: None,
            },
        ]
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: everything_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: reqs(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: everything_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: reqs(),
            }),
    )
}

fn everything_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kind = match reg.interner().lookup("everything") {
        Some(s) => CounterKind::Named(s),
        None => return Vec::new(),
    };
    let mut effects = Vec::new();
    for t in trig.targets.targets.iter() {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: kind.clone(),
                count: 1,
            });
        }
    }
    // GAP: "each land/creature with an everything counter is every land/creature
    // type" — a counter-keyed continuous type-granting static is not expressible.
    effects
}
