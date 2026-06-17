//! Groaaaaag, Hungry Monster — `{4}{B}{G}` 4/4 Legendary Ooze.
//! (Commander Suspend 4 — {B}{G} is not in the usable keyword surface — GAP.)
//! "Whenever Groaaaaag, Hungry Monster attacks, you may sacrifice any number of other
//! creatures. Draw that many cards, gain that much life, and put that many +1/+1 counters
//! on Groaaaaag."

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
    let name = reg.interner_mut().intern("Groaaaaag, Hungry Monster");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attack(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may sacrifice any number of other creatures. Draw THAT MANY cards, gain
    // THAT MUCH life, and put THAT MANY +1/+1 counters on Groaaaaag." The payoff amounts
    // are tied to the count chosen in the same sacrifice; no Effect can thread the
    // player-chosen sacrifice count into the subsequent draw/gain/counter amounts.
    Vec::new()
}
