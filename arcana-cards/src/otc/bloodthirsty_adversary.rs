//! Bloodthirsty Adversary — `{1}{R}` 2/2 red Vampire.
//! Haste.
//! When this creature enters, you may pay {2}{R} any number of times. When
//! you pay this cost one or more times, put that many +1/+1 counters on this
//! creature, then exile up to that many target instant and/or sorcery cards
//! with mana value 3 or less from your graveyard and copy them. You may cast
//! any number of the copies without paying their mana costs.

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
    let name = reg.interner_mut().intern("Bloodthirsty Adversary");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: pay_any_number_of_times,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pay_any_number_of_times(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {2}{R} any number of times" then a dynamic chain keyed
    // on the times paid (put that many +1/+1 counters, then exile up to that
    // many target I/S cards mv<=3 from your graveyard and copy them, cast the
    // copies free). OptionalPayment is a single yes/no pay; there is no
    // pay-N-times primitive and no dynamic-x accessor for the times paid.
    Vec::new()
}
