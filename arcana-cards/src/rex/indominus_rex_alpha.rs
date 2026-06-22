//! Indominus Rex, Alpha — `{1}{U/B}{U/B}{G}{G}` 6/6 Legendary Dinosaur
//! Mutant (B/G/U).
//!
//! * "As Indominus Rex enters, discard any number of creature cards. It
//!   enters with a flying counter on it if a card discarded this way
//!   has flying. The same is true for first strike, double strike,
//!   deathtouch, hexproof, haste, indestructible, lifelink, menace,
//!   reach, trample, and vigilance." — GAP. This is an "as enters"
//!   replacement that discards a variable number of cards and conditions
//!   each keyword counter on a discarded card carrying that keyword.
//!   No primitive composes discard → per-keyword conditional counter.
//! * "When Indominus Rex enters, draw a card for each counter on it."
//!
//! GAP (ETB draw count): "for each counter on it" sums counters of EVERY
//! kind on the source. The allowed `state` surface only exposes a
//! per-kind `count_counters(kind)` read (no total-counter script
//! helper), so the dynamic amount can't be computed; a literal would be
//! a materially wrong card. The ETB trigger is wired with a GAP'd
//! effect.

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
    let name = reg.interner_mut().intern("Indominus Rex, Alpha");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U/B}{U/B}{G}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: draw_per_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_per_counter(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "draw a card for each counter on it" — no total-counter
    // (all-kinds) script helper to compute the dynamic amount.
    Vec::new()
}
