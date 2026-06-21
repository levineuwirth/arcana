//! Territory Culler — `{4}{G}` 7/5 Eldrazi with Reach. Devoid (colorless).
//!
//! Oracle:
//! * Devoid (this card has no color)
//! * Reach
//! * Landfall — Whenever a land you control enters, look at the top card
//!   of your library. If it's a creature card, you may reveal it and put
//!   it into your hand. If you don't put the card into your hand, you may
//!   put it into your graveyard.
//!
//! Devoid is modelled by `colors: ColorSet::colorless()`. Neither Devoid
//! nor Landfall is a KeywordAbility variant, so the keyword line is just
//! Reach. The landfall trigger is a land-ETB ZoneChange whose effect is a
//! single-card dig (creature filter, rest → graveyard).

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Territory Culler");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    let lands_you_control = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: lands_you_control,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: landfall_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn landfall_dig(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 1,
        filter: Some(ObjectFilter::creature()),
        rest: DigRest::Graveyard,
    }]
}
