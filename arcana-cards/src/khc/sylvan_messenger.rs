//! Sylvan Messenger — `{3}{G}` 2/2 Elf with Trample.
//!
//! Oracle:
//! * Trample — keyword line.
//! * When this creature enters, reveal the top four cards of your
//!   library. Put all Elf cards revealed this way into your hand and the
//!   rest on the bottom of your library in any order.
//!
//! GAP fidelity: "reveal the top four, put ALL Elf cards into your hand,
//! rest to the bottom" is not expressible — `DigTopN` takes a single
//! card and `RevealUntil` stops at the first match, so neither models a
//! take-all-matching-from-a-fixed-window. Approximated as a
//! `TutorToHand` for an Elf card (matches Kavu Howler's catalog
//! treatment of the identical template).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sylvan Messenger");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_elves_to_hand,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_elves_to_hand(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal top 4, put ALL Elf cards into hand, rest to bottom" — a
    // take-all-matching-from-a-fixed-window is not expressible; approximated
    // with a TutorToHand for an Elf card.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: script::subtype_filter(reg, "Elf"),
        reveal: true,
    }]
}
