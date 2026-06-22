//! Aven Heartstabber — `{U}{B}` 1/1 Creature — Bird Assassin.
//! Flying
//! As long as there are five or more mana values among cards in your
//! graveyard, this creature gets +2/+2 and has deathtouch.
//! When this creature dies, mill two cards, then draw a card.
//!
//! Flying is a base keyword. The conditional anthem static (+2/+2 and
//! deathtouch while 5+ distinct mana values are in your graveyard) gates on
//! distinct-mana-value counting with no demonstrated condition/static
//! primitive and is GAP'd. The dies trigger mills two then draws a card.

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
    let name = reg.interner_mut().intern("Aven Heartstabber");
    let bird = reg.interner_mut().intern("Bird");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "As long as there are five or more mana values among cards in your
    // graveyard, this creature gets +2/+2 and has deathtouch." — conditional
    // anthem static gated on distinct-mana-value counting, no primitive.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_mill_then_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_mill_then_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Mill {
            player: trig.controller,
            count: 2,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ]
}
