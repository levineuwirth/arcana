//! Kardum, Patron of Flames — `{2}{R}{R}` 4/3 Legendary Creature — Demon, red.
//! Haste.
//! Whenever Kardum attacks, put a flame counter on it, then seek a card with
//! mana value equal to the number of flame counters on it and exile that card
//! face down.
//! When Kardum dies, put all cards you own exiled with it into your hand. At
//! the beginning of the end step of your next turn, discard those cards.
//!
//! Scryfall lists "Seek" as a keyword, but there is no `KeywordAbility::Seek`
//! variant — it is dropped; only `Haste` is expressible.
//! Trigger 1 expresses the "put a flame counter on it" portion; the "seek a
//! card ... exile that card face down" portion has no matching `Effect`
//! variant (GAP). Trigger 2 has no `Effect` variant to return cards exiled
//! with this card to hand nor to discard "those cards" (GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kardum, Patron of Flames");
    let demon = reg.interner_mut().intern("Demon");
    // Intern the named "flame" counter so the resolver can recover it.
    let _flame = reg.interner_mut().intern("flame");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
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
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Whenever Kardum attacks, put a flame counter on it, then seek a card with
/// mana value equal to the number of flame counters on it and exile that card
/// face down." Expresses the flame-counter placement; the seek/exile portion
/// has no matching `Effect` variant.
fn on_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "seek a card ... exile that card face down" has no Effect::Seek variant
    match reg.interner().lookup("flame") {
        Some(s) => vec![Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Named(s),
            count: 1,
        }],
        None => Vec::new(),
    }
}

/// "When Kardum dies, put all cards you own exiled with it into your hand. At
/// the beginning of the end step of your next turn, discard those cards."
fn on_dies(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect variant for "return cards exiled with ~ to hand" nor delayed "discard those cards"
    Vec::new()
}
