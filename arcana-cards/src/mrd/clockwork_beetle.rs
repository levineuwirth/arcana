//! Clockwork Beetle — `{1}` 0/0 Artifact Creature — Insect.
//! "This creature enters with two +1/+1 counters on it."
//! "Whenever this creature attacks or blocks, remove a +1/+1 counter from it at
//! end of combat."
//!
//! The enters-with-two-counters clause is implemented as an ETB trigger adding
//! two +1/+1 counters to itself. The attack-or-blocks → delayed end-of-combat
//! removal is GAP'd: there is no "attacks or blocks" trigger condition (only
//! SelfAttacks / SelfBlocks / SelfBlocksOrBecomesBlocked), and no documented way
//! to schedule a counter removal at end of combat.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Clockwork Beetle");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_two_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
    // GAP: "Whenever this creature attacks or blocks, remove a +1/+1 counter
    // from it at end of combat" — no "attacks or blocks" trigger condition and no
    // documented way to schedule the at-end-of-combat counter removal.
}

fn etb_two_counters(
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
