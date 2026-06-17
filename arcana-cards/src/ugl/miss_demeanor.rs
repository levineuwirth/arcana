//! Miss Demeanor — `{3}{W}` 3/1 white Lady of Proper Etiquette with
//! Flying and First strike.
//! "At the beginning of each other player's upkeep, you may compliment
//! that player on their game play. If you don't, sacrifice this creature."
//!
//! The "compliment" action is an Un-set, real-world social interaction
//! that has no engine representation, so the upkeep trigger's conditional
//! sacrifice cannot be expressed faithfully — see GAP. Keywords emitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Miss Demeanor");
    let lady = reg.interner_mut().intern("Lady of Proper Etiquette");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lady);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_triggered_ability(
        TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: compliment_or_sacrifice,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        },
    ))
}

fn compliment_or_sacrifice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may compliment that player ... if you don't, sacrifice
    // this creature" — the compliment is an out-of-game social action
    // with no engine representation, so the conditional sacrifice cannot
    // be modeled.
    Vec::new()
}
