//! Cuboid Colony — `{G}{U}` 1/1 green-blue Insect.
//!
//! * Flash, Flying, Trample (keywords).
//! * Increment — "Whenever you cast a spell, if the amount of mana you
//!   spent is greater than this creature's power or toughness, put a
//!   +1/+1 counter on this creature." Modeled as a SpellCast (you)
//!   trigger that adds a +1/+1 counter. The intervening-if comparison
//!   (mana spent vs. this creature's power/toughness) has no condition
//!   helper, so it is GAP'd and the trigger fires unconditionally.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cuboid Colony");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // Increment is not a supported keyword variant; its ability is
        // modeled as the triggered ability below.
        keywords: vec![
            KeywordAbility::Flash,
            KeywordAbility::Flying,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                // GAP: intervening-if "if the amount of mana you spent is
                // greater than this creature's power or toughness" has no
                // condition helper; fires unconditionally.
                intervening_if: None,
                effect: increment_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn increment_counter(
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
