//! Primal Adversary — `{2}{G}` 4/3 green Wolf.
//!
//! Trample.
//! When this creature enters, you may pay {1}{G} any number of times.
//! When you pay this cost one or more times, put that many +1/+1
//! counters on this creature, then up to that many target lands you
//! control become 3/3 Wolf creatures with haste that are still lands.
//!
//! The ETB trigger fires, but its payload is gated on a REPEATED
//! optional payment ("pay {1}{G} any number of times") whose count
//! then scales the counters and the number of land targets. The
//! demonstrated `OptionalPayment` is a single yes/no gate, not a
//! repeated-N payment, so the variable-count counters + land-animation
//! payload is GAP'd.

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
    let name = reg.interner_mut().intern("Primal Adversary");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_pay_repeatedly,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_pay_repeatedly(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "pay {1}{G} any number of times" is a repeated-N optional
    // payment whose count scales +1/+1 counters and the number of
    // target lands animated into 3/3 Wolves; no repeated-payment shape
    // in the demonstrated API.
    Vec::new()
}
