//! Yanling's Harbinger — `{3}{U}{U}` 2/4 Bird with Flying.
//! When this creature enters, you may search your library and/or graveyard
//! for a card named Mu Yanling, Celestial Wind, reveal it, and put it into
//! your hand. If you search your library this way, shuffle.

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
    let name = reg.interner_mut().intern("Yanling's Harbinger");
    let bird = reg.interner_mut().intern("Bird");
    let _target = reg.interner_mut().intern("Mu Yanling, Celestial Wind");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_yanling,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_yanling(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "and/or graveyard" half of the search is not expressible
    // alongside a library tutor — modeling the library search by name.
    let nm = reg.interner().lookup("Mu Yanling, Celestial Wind");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
        reveal: true,
    }]
}
