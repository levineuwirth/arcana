//! Mindbender Spores — `{2}{G}` 0/1 Fungus Wall with Defender and Flying.
//! Whenever this creature blocks a creature, put four fungus counters on that
//! creature. The creature gains "This creature doesn't untap during your untap
//! step if it has a fungus counter on it" and "At the beginning of your upkeep,
//! remove a fungus counter from this creature."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mindbender Spores");
    let fungus = reg.interner_mut().intern("Fungus");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(wall);

    // Intern the named counter so the kind exists by resolve time.
    let _fungus_counter = reg.interner_mut().intern("fungus");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBlocks,
            intervening_if: None,
            effect: fungus_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn fungus_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(other) = trig.other_combatant() else {
        return Vec::new();
    };
    let kind = reg
        .interner()
        .lookup("fungus")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    // GAP: the granted abilities ("doesn't untap if it has a fungus counter" and
    // "at the beginning of your upkeep, remove a fungus counter") cannot be granted
    // by an Effect here — GrantTriggeredAbility could carry the upkeep-removal
    // trigger but not the don't-untap static, and the removal trigger has no
    // self-counter accessor in a granted body; so only the counter placement is
    // emitted.
    vec![Effect::AddCounters {
        target: other,
        kind,
        count: 4,
    }]
}
