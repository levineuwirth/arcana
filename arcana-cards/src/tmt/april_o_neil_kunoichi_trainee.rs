//! April O'Neil, Kunoichi Trainee — `{1}{W}` 2/2 Legendary Human Ninja.
//!
//! When April O'Neil enters, scry 2.
//! April O'Neil can't be blocked by creatures with power 3 or greater.
//!
//! The ETB scry is wired. The Scryfall "Scry" keyword is reminder for that
//! ETB, not a usable `KeywordAbility` variant. GAP: "can't be blocked by
//! creatures with power 3 or greater" is a continuous, condition-restricted
//! evasion static — `Effect::CantBeBlocked` is an unconditional grant, not
//! a power-thresholded blocking restriction, so this static is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("April O'Neil, Kunoichi Trainee");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: "can't be blocked by creatures with power 3 or greater" —
    // condition-restricted blocking static, no demonstrated primitive.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_scry_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_scry_two(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry {
        player: trig.controller,
        count: 2,
    }]
}
