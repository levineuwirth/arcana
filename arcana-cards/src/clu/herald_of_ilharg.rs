//! Herald of Ilharg — `{2}{R}{G}` 3/3 Boar Beast with Trample.
//!
//! Oracle:
//! * Trample — keyword, base characteristic.
//! * "Whenever you cast a creature spell, put two +1/+1 counters on this
//!    creature. If that spell has mana value 5 or greater, this creature deals
//!    damage equal to the number of counters on it to each opponent." — the
//!    counter half is wired (two +1/+1 counters on self). GAP (partial): the
//!    "if that spell has mana value 5 or greater" rider needs the cast spell's
//!    mana value (no accessor), so the conditional damage-to-each-opponent
//!    clause is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Herald of Ilharg");
    let boar = reg.interner_mut().intern("Boar");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: add_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn add_counters(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP (partial): "If that spell has mana value 5 or greater, deal damage
    //      equal to the number of counters on this to each opponent." — the cast
    //      spell's mana value is not readable, so the conditional damage clause
    //      is omitted.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
