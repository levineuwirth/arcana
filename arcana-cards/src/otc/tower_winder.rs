//! Tower Winder — `{1}{G}` 1/1 Snake with Reach and Deathtouch.
//! "When this creature enters, search your library and/or graveyard for a
//!  card named Command Tower, reveal it, and put it into your hand. If you
//!  search your library this way, shuffle."
//!
//! The library search-by-name to hand is expressible via TutorToHand with
//! a name filter (shuffle is automatic). The graveyard alternative is not
//! expressible (no graveyard-search-by-name-to-hand), so only the library
//! search is wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tower Winder");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_command_tower,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_command_tower(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "and/or graveyard" — no graveyard-search-by-name-to-hand effect;
    // only the library search is wired.
    let nm = reg.interner().lookup("Command Tower");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
        reveal: true,
    }]
}
