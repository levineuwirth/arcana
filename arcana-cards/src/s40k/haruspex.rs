//! Haruspex — `{3}{G}` 2/2 Tyranid.
//! Rapacious Hunger — "Whenever another creature dies, put a +1/+1 counter
//!   on this creature."
//! Devouring Monster — "{T}, Remove X +1/+1 counters from this creature:
//!   Add X mana of any one color."
//!
//! The death trigger is modeled with a battlefield→graveyard ZoneChange on
//! creatures; engine self-exclusion for "another" is not expressible via
//! the filter (documented limitation), so it may also see its own death.
//!
//! GAP (activated ability): "Remove X +1/+1 counters: Add X mana of any
//! one color." — the ActivationCost remove-counter field takes a fixed
//! count (not a variable X), and there is no any-color X-scaled AddMana
//! primitive. The whole activated ability is GAP'd rather than emit a
//! fixed-count, fixed-color approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haruspex");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: activated ability "{T}, Remove X +1/+1 counters: Add X mana of
    // any one color" — variable-X counter removal cost + any-color X mana
    // are both unexpressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature(),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: grow_on_death,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn grow_on_death(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
