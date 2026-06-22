//! Spellgorger Barbarian — `{3}{R}` 3/1 Human Nightmare Barbarian (red).
//!
//! Oracle:
//! * "When this creature enters, discard a card at random." — ETB trigger,
//!   modeled with `Discard { choice: DiscardChoice::Random }`.
//! * "When this creature leaves the battlefield, draw a card." — there is no
//!   exact "leaves the battlefield" TriggerCondition; `SelfDies` is the closest
//!   available variant (covers dying to the graveyard but not bounce/exile
//!   leaves). GAP'd in part for those zones; the draw is emitted.

use arcana_core::effects::{DiscardChoice, Effect};
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
    let name = reg.interner_mut().intern("Spellgorger Barbarian");
    let human = reg.interner_mut().intern("Human");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(nightmare);
    subtypes.0.insert(barbarian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard_random,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: trigger — "leaves the battlefield" has no exact
            // TriggerCondition; SelfDies is the closest variant (misses
            // bounce/exile leaves).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: leaves_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_discard_random(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Discard {
        player: trig.controller,
        count: 1,
        choice: DiscardChoice::Random,
    }]
}

fn leaves_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
