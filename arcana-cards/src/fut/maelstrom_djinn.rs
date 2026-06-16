//! Maelstrom Djinn — `{7}{U}` 5/6 Djinn (blue) with Flying and Morph.
//!
//! Oracle:
//! * Flying — keyword.
//! * "Morph {2}{U}" — the Morph keyword. There is no
//!   `KeywordAbility::Morph` variant and the face-down-cast / turn-face-up
//!   mechanic is not expressible on this card class, so the morph cost is
//!   GAP'd (only Flying is recorded in `keywords`).
//! * "When this creature is turned face up, put two time counters on it and
//!   it gains vanishing." — the TURNED-FACE-UP trigger has no matching
//!   `TriggerCondition`; `SelfTransforms` is the closest (morph flip is a
//!   transform-like face swap) but the engine has no turned-face-up event,
//!   so the trigger is GAP'd. The "put two time counters on it" portion
//!   would be expressible (AddCounters Time), but "it gains vanishing"
//!   (granting a parametrized keyword as a continuous effect) is not, and
//!   firing on the wrong event would be materially wrong — so the whole
//!   effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maelstrom Djinn");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "when this creature is turned face up" has no
                // matching TriggerCondition; SelfTransforms is the closest but
                // there is no turned-face-up (morph-flip) event.
                trigger_condition: TriggerCondition::SelfTransforms { to_face: None },
                intervening_if: None,
                effect: on_turned_face_up,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_turned_face_up(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "it gains vanishing" (granting a parametrized keyword as a
    // continuous effect) is unexpressible, and the trigger fires on the
    // wrong (transform) event, so the whole effect is GAP'd.
    Vec::new()
}
