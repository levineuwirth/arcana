//! Search for Dagger — `{1}{W}` enchantment.
//! "Whenever your commander enters or attacks, look at the top six
//! cards of your library. You may reveal a legendary creature card
//! from among them and put it into your hand. Put the rest on the
//! bottom of your library in a random order."
//!
//! GAP: the engine has no commander concept, so "your commander"
//! cannot be filtered — both triggers (enters / attacks) are wired
//! over ANY creature you control, which over-fires. The dig itself is
//! faithful: `DigTopN` 6 with a legendary-creature filter, rest to
//! the bottom in random order.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Search for Dagger");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "your commander enters" — no commander concept;
                // fires for any creature entering under your control.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: dig_for_dagger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: "your commander attacks" — same commander gap;
                // fires for any attacking creature you control.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: dig_for_dagger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…look at the top six cards of your library. You may reveal a
/// legendary creature card from among them and put it into your hand.
/// Put the rest on the bottom of your library in a random order."
fn dig_for_dagger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 6,
        filter: Some(
            ObjectFilter::creature()
                .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
        ),
        rest: DigRest::BottomRandom,
    }]
}
