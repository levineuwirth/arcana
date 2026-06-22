//! Pensive Professor — `{1}{U}{U}` 0/2 Creature — Human Wizard.
//!
//! Increment (Whenever you cast a spell, if the amount of mana you spent is
//! greater than this creature's power or toughness, put a +1/+1 counter on
//! this creature.)
//! Whenever one or more +1/+1 counters are put on this creature, draw a card.
//!
//! # GAP
//! "Increment" is not in the supported KeywordAbility surface, and its
//! reminder-text trigger depends on "the amount of mana you spent" — there is
//! no mana-spent accessor on the trigger event, so the Increment counter-add
//! is GAP'd (it is therefore not emitted as a keyword variant). The
//! counters-put trigger below is fully expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pensive Professor");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CounterAdded {
                on: TriggerSelf::Source,
                kind: Some(CounterKind::PlusOnePlusOne),
                chapter: None,
            },
            intervening_if: None,
            effect: draw_on_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_on_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
