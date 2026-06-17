//! Garruk's Warsteed — `{3}{G}{G}` 3/5 Rhino with Vigilance.
//!
//! When this creature enters, search your library (and/or graveyard)
//! for a card named Garruk, Savage Herald, reveal it, and put it into
//! your hand. The library search is modeled via `TutorToHand` with a
//! name filter (auto-shuffles). The graveyard-search half has no
//! demonstrated "return-by-name from graveyard" primitive, so it is
//! GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Garruk's Warsteed");
    let rhino = reg.interner_mut().intern("Rhino");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: tutor_garruk,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn tutor_garruk(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "and/or graveyard" search half has no return-by-name-from-graveyard primitive.
    let nm = reg.interner().lookup("Garruk, Savage Herald");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        reveal: true,
    }]
}
