//! Fetid Gargantua — `{4}{B}` 4/4 Horror.
//!
//! Oracle:
//! * "{2}{B}: Adapt 2. (If this creature has no +1/+1 counters on it,
//!   put two +1/+1 counters on it.)" — an activated ability. There is
//!   no Effect::Adapt and no "if it has no +1/+1 counters" precondition,
//!   so it is modeled as a {2}{B} activation that places two +1/+1
//!   counters on this creature; the "only if it has none" guard is a
//!   documented fidelity gap.
//! * "Whenever one or more +1/+1 counters are put on this creature, you
//!   may draw two cards. If you do, you lose 2 life." — a CounterAdded
//!   trigger. The "you may ... if you do" optionality has no free-may
//!   primitive, so the draw-2 / lose-2 are sequenced unconditionally
//!   (the may-optionality is a documented fidelity gap).
//!
//! Note: the Scryfall keyword "Adapt" is an inert marker in the engine
//! (it synthesizes no ability), so it is omitted in favor of the
//! explicit activated ability above.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fetid Gargantua");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}: Adapt 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: adapt_two,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: counter_added_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn adapt_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP fidelity: "if this creature has no +1/+1 counters on it" guard
    // is not expressible; always places the two counters.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}

fn counter_added_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP fidelity: "you may draw two cards. If you do, you lose 2 life"
    // has no free-may primitive; the draw and life loss are sequenced
    // unconditionally.
    vec![Effect::Sequence(vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 2,
        },
        Effect::LoseLife {
            player: trig.controller,
            amount: 2,
        },
    ])]
}
