//! Rescuer Sphinx — `{2}{U}{U}` 3/2 blue Sphinx with Flying.
//!
//! * Flying — keyword.
//! * "As this creature enters, you may return a nonland permanent you
//!   control to its owner's hand. If you do, this creature enters with
//!   a +1/+1 counter on it." — modeled as an ETB trigger (the engine
//!   has no as-enters replacement primitive). GAP below.

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
    let name = reg.interner_mut().intern("Rescuer Sphinx");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enters_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn enters_return(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "As this creature enters, you MAY return a nonland permanent
    // you control to its owner's hand; IF YOU DO, it enters with a
    // +1/+1 counter" — an as-enters replacement whose optional bounce
    // gates the counter. No primitive ties an optional ReturnToHand of
    // a chosen permanent to a conditional self-counter (OptionalPayment
    // only takes Mana/Life costs), and the counter is an enters-with
    // replacement, not a post-ETB add. Whole effect GAP'd.
    Vec::new()
}
