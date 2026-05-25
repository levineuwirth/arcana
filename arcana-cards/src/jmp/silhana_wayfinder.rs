//! Silhana Wayfinder — `{1}{G}` 2/1 green Elf Scout creature.
//! "When this creature enters, look at the top four cards of your library. You may reveal a
//! creature or land card from among them and put it on top of your library. Put the rest on
//! the bottom of your library in a random order."
//!
//! # Notes
//! GAP: "look at top 4, choose creature or land, rest to bottom random" — no look-and-choose-
//! from-top-N effect in engine. Using Scry 1 as closest approximation of library manipulation.

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
    let name = reg.interner_mut().intern("Silhana Wayfinder");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_look_top_four,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_look_top_four(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top 4, choose creature or land for top, rest to bottom random" —
    // no look-and-choose-from-top-N effect. Using Scry 1 as best approximation.
    vec![Effect::Scry { player: trig.controller, count: 1 }]
}
