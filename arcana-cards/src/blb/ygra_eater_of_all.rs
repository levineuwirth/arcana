//! Ygra, Eater of All — `{3}{B}{G}` 6/6 Legendary Creature — Elemental
//! Cat.
//!
//! Oracle:
//! * "Ward—Sacrifice a Food." — GAP: non-mana Ward cost is not
//!   expressible (only `Ward(ManaCost)` exists); emit no keyword.
//! * "Other creatures are Food artifacts in addition to their other
//!   types and have '{2}, {T}, Sacrifice this permanent: You gain 3
//!   life.'" — GAP: a continuous static type/ability-granting effect,
//!   not a triggered/activated ability.
//! * "Whenever a Food is put into a graveyard from the battlefield, put
//!   two +1/+1 counters on Ygra." — a battlefield→graveyard ZoneChange
//!   trigger filtered to Food permanents.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ygra, Eater of All");
    let elemental = reg.interner_mut().intern("Elemental");
    let cat = reg.interner_mut().intern("Cat");
    let _food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: "Ward—Sacrifice a Food" — non-mana Ward not expressible.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: food_filter(reg),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: add_two_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn food_filter(reg: &CardRegistry) -> arcana_core::targets::ObjectFilter {
    script::subtype_filter(reg, "Food")
}

fn add_two_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
