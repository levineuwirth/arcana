//! Marauding Mako — `{R}` 1/1 Shark Pirate with Cycling {2}.
//! "Whenever you discard one or more cards, put that many +1/+1 counters on
//!  this creature." — the CardDiscarded trigger fires, but "that many"
//!  (the count of cards discarded by the triggering event) is not exposed
//!  on PendingTrigger, so the dynamic counter amount is GAP'd.
//! "Cycling {2}" — wired from the keyword; the engine synthesizes the
//!  "{2}, Discard this card: Draw a card" activated ability.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marauding Mako");
    let shark = reg.interner_mut().intern("Shark");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shark);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: counters_on_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counters_on_discard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put that many +1/+1 counters" — the number of cards discarded by
    // the triggering event is not exposed on PendingTrigger, so the dynamic
    // amount cannot be computed. Emitting a literal would be a wrong card.
    Vec::new()
}
