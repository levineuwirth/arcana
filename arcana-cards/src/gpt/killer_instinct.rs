//! Killer Instinct — `{4}{R}{G}` enchantment. "At the beginning of
//! your upkeep, reveal the top card of your library. If it's a
//! creature card, put it onto the battlefield. That creature gains
//! haste until end of turn. Sacrifice it at the beginning of the next
//! end step."
//!
//! The reveal-and-deploy core is modeled with `Effect::RevealUntil`
//! capped at one card; the haste grant and the delayed end-step
//! sacrifice are GAP'd (the entering card's object id is not available
//! to the resolver), and a non-creature reveal goes to the bottom
//! rather than staying on top (documented fidelity gap).

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Killer Instinct");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}").expect("valid cost")),
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
                effect: reveal_and_deploy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "Reveal the top card of your library. If it's a creature card, put
/// it onto the battlefield. …"
fn reveal_and_deploy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "That creature gains haste until end of turn. Sacrifice it
    // at the beginning of the next end step." — the card put onto the
    // battlefield by RevealUntil has no readable object id here, so
    // neither the haste grant nor the DelayedAction sacrifice can be
    // attached to it.
    // GAP (fidelity): a revealed non-creature card should stay on top
    // of the library; DigRest has no leave-on-top option, so it goes to
    // the bottom instead.
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: Some(1),
    }]
}
