//! Murder of Crows — `{3}{U}{U}` 4/4 Creature — Bird.
//!
//! Flying
//! * Whenever another creature dies, you may draw a card. If you do, discard
//!   a card.
//!
//! GAP (fidelity): the "you may draw a card. If you do, discard a card."
//! optional is modeled as an unconditional draw-then-discard (the "may" is a
//! resolution-time choice with no primitive). The "another creature"
//! self-exclusion is also not expressible via ZoneChange's filter.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Murder of Crows");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature(),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: loot_on_death,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn loot_on_death(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
