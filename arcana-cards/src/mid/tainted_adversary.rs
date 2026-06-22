//! Tainted Adversary — `{1}{B}` 2/3 Zombie with Deathtouch.
//!
//! Oracle:
//! * Deathtouch
//! * When this creature enters, you may pay {2}{B} any number of times.
//!   When you pay this cost one or more times, put that many +1/+1
//!   counters on this creature, then create twice that many 2/2 black
//!   Zombie creature tokens with decayed.
//!
//! Modeled as an ETB trigger offering a single optional `{2}{B}`
//! payment. On payment, one +1/+1 counter is added and two 2/2 black
//! Zombie tokens are created (= twice the one payment).
//!
//! PARTIAL / GAPs:
//! * "any number of times" — `OptionalPayment` is a single yes/no gate,
//!   not a repeatable count; only one iteration is modeled.
//! * "decayed" — not a usable `KeywordAbility` variant, so the tokens
//!   are plain 2/2 black Zombies without the decayed rider.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tainted Adversary");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_pay,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_pay(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let mut zsubs = SubtypeSet::default();
    zsubs.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: zsubs,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}{B}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![
            Effect::AddCounters {
                target: trig.source,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            },
            Effect::CreateToken { controller: trig.controller, token: token.clone() },
            Effect::CreateToken { controller: trig.controller, token },
        ])),
        else_effect: None,
    }]
}
