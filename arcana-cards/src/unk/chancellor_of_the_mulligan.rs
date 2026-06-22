//! Chancellor of the Mulligan — `{7}` 7/7 Artifact Creature — Phyrexian Construct.
//! You may reveal this card from your opening hand; if you do, at the
//!   beginning of the first upkeep, shuffle it into your library and draw a
//!   card. (opening-hand reveal — GAP)
//! Menace.
//! When Chancellor of the Mulligan enters, draw three cards. When you do,
//!   any number of target players mulligan.

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
    let name = reg.interner_mut().intern("Chancellor of the Mulligan");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    // GAP: the opening-hand reveal ability ("at the beginning of the
    // first upkeep, shuffle it in and draw") is not an expressible
    // mechanic.
    reg.register(
        CardDefinition::new(name, chars)
            // When Chancellor enters, draw three cards.
            // GAP: the "When you do, any number of target players
            // mulligan" rider has no mulligan effect primitive.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: draw_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_three(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 3 }]
}
