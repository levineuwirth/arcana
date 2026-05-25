//! Kavu Howler — `{4}{G}{G}` 4/5 green Creature — Kavu.
//! "When this creature enters, reveal the top four cards of your library.
//! Put all Kavu cards revealed this way into your hand and the rest on the
//! bottom of your library in any order."
//! GAP: conditional reveal + type-sort not expressible; using TutorToHand
//! with Kavu subtype filter as approximation (misses the "rest to bottom"
//! and the exact top-4 limit).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kavu Howler");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_kavu_to_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_kavu_to_hand(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal top 4, put Kavu into hand, rest to bottom" not expressible;
    // using TutorToHand Kavu as approximation
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: script::subtype_filter(reg, "Kavu"),
        reveal: true,
    }]
}
