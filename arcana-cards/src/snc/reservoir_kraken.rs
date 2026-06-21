//! Reservoir Kraken — `{2}{U}{U}` 6/6 Kraken with Trample and Ward {2}.
//!
//! Rules text:
//! * Trample, ward {2}.
//! * "At the beginning of each combat, if this creature is untapped,
//!   any opponent may tap an untapped creature they control. If they
//!   do, tap this creature and create a 1/1 blue Fish creature token
//!   with 'This token can't be blocked.'" — GAP'd: the body is an
//!   opponent's may-tap choice with a conditional follow-up (tap self
//!   + token) that has no expressible primitive, and the "if this
//!   creature is untapped" intervening-if has no condition helper.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reservoir Kraken");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::Any,
                },
                // GAP: intervening-if "if this creature is untapped" — no
                // source-untapped condition helper.
                intervening_if: None,
                effect: combat_opponent_tap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_opponent_tap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "any opponent may tap an untapped creature they control. If
    // they do, tap this creature and create a 1/1 blue Fish token with
    // 'can't be blocked'." — opponent may-tap with conditional follow-up
    // has no expressible primitive.
    Vec::new()
}
