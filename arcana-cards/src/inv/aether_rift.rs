//! Aether Rift — `{1}{R}{G}` enchantment.
//! "At the beginning of your upkeep, discard a card at random. If you
//! discard a creature card this way, return it from your graveyard to
//! the battlefield unless any player pays 5 life."
//!
//! The random discard is faithful. GAP: the reflexive "If you discard
//! a creature card this way, return IT … unless any player pays 5
//! life" clause needs the identity of the just-discarded card (not
//! observable from the resolver) and an any-player payment window;
//! that half is omitted.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aether Rift");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: random_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…discard a card at random. If you discard a creature card this
/// way, return it from your graveyard to the battlefield unless any
/// player pays 5 life."
fn random_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the reflexive return-unless-any-player-pays-5-life clause is
    // not expressible — the just-discarded card's identity is not
    // observable and there is no any-player payment prompt.
    vec![Effect::Discard {
        player: trig.controller,
        count: 1,
        choice: DiscardChoice::Random,
    }]
}
