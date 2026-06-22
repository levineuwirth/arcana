//! Moss-Pit Skeleton — `{B}{G}` 2/2 Plant Skeleton.
//! "Kicker {3}"
//! "If this creature was kicked, it enters with three +1/+1 counters on it."
//! "Whenever one or more +1/+1 counters are put on a creature you control, if
//!  this card is in your graveyard, you may put this card on top of your library."
//!
//! GAP: Kicker is not an available KeywordAbility variant; the kicker keyword and
//! the kicker-conditional "enters with three +1/+1 counters" are not expressible
//! and are omitted.
//! Implemented: the graveyard recursion trigger — it watches +1/+1 counters
//! placed on creatures you control and (firing from the graveyard, which encodes
//! the "if this card is in your graveyard" gate) puts this card on top of your
//! library.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moss-Pit Skeleton");
    let plant = reg.interner_mut().intern("Plant");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(skeleton);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CounterAdded {
                on: TriggerSelf::AnyMatching(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                kind: Some(CounterKind::PlusOnePlusOne),
                chapter: None,
            },
            intervening_if: None,
            effect: put_self_on_top,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn put_self_on_top(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PutOnTopOfLibrary { target: trig.source }]
}
