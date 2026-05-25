//! Sand Golem — `{5}` 3/3 colorless Artifact Creature — Golem.
//! "When a spell or ability an opponent controls causes you to discard this card, return
//! this card from your graveyard to the battlefield with a +1/+1 counter on it at the
//! beginning of the next end step."
//!
//! # Notes
//! GAP: trigger fires when this card is discarded due to opponent — trigger_zones should
//! include hand/exile; using Graveyard(0) as approximation.
//! GAP: "at the beginning of the next end step" delayed return from graveyard — DelayedAction
//! does not support ReturnFromGraveyardToBattlefield. Using ReturnFromGraveyardToBattlefield
//! immediately as best approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sand Golem");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "opponent causes you to discard this card" — using CardDiscarded
                // opponent trigger as best approximation.
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: discarded_return,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn discarded_return(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: delayed "at beginning of next end step" return — using immediate return.
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: trig.source },
        Effect::AddCounters { target: trig.source, kind: CounterKind::PlusOnePlusOne, count: 1 },
    ]
}
