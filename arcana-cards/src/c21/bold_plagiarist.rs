//! Bold Plagiarist — `{3}{B}` 2/2 black Vampire Rogue.
//!
//! Oracle:
//! * Flash.
//! * Whenever an opponent puts one or more counters on a creature they
//!   control, they put the same number and kind of counters on this
//!   creature.
//!
//! Decomposition: Flash → `keywords`. The counter-mirroring trigger
//! fires on counters added to an OPPONENT'S creature (not this one) and
//! must replicate the exact number and kind onto Bold Plagiarist — the
//! `CounterAdded` trigger variant only exposes a `TriggerSelf` watch
//! target (Source) and no way to read the triggering event's added
//! count/kind, so this ability is not expressible with the demonstrated
//! API. The trigger is emitted with the closest condition and a GAP'd
//! resolver.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bold Plagiarist");
    let vampire = reg.interner_mut().intern("Vampire");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![arcana_core::effects::KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // Closest available condition. The oracle watches an
            // opponent placing counters on a creature THEY control; this
            // variant only watches counters on the source. Used as a
            // placeholder for the GAP'd resolver.
            trigger_condition: TriggerCondition::CounterAdded {
                on: TriggerSelf::Source,
                kind: None,
                chapter: None,
            },
            intervening_if: None,
            effect: mirror_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn mirror_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "they put the same number and kind of counters on this
    // creature" — requires reading the triggering counter-add event's
    // added count and kind (which opponent's creature, how many, what
    // kind) and replicating them onto the source. No PendingTrigger
    // accessor exposes the added counter count/kind, and the
    // CounterAdded condition cannot watch counters placed on an
    // opponent's other creature. Not expressible with the demonstrated
    // API.
    Vec::new()
}
