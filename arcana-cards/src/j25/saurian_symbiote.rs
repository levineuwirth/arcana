//! Saurian Symbiote — `{3}{G}` 2/3 Fungus Dinosaur.
//!
//! Rules text:
//! * Reach
//! * When this creature enters, choose one —
//!     • Put a +1/+1 counter on this creature.
//!     • Create a 1/1 green Saproling creature token.
//!
//! The Reach keyword is faithful. The ETB ability is a MODAL "choose one" on a
//! TRIGGERED ability; the demonstrated modal machinery (ModalSpec /
//! dispatch_modal_effect / with_mode_effects) is only wired for SPELL abilities,
//! and TriggeredAbilityDef has no modal field, so the mode selection can't be
//! posted from a trigger. The trigger is registered; its effect is GAP'd
//! (emitting one mode unconditionally would be wrong).

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
    let name = reg.interner_mut().intern("Saurian Symbiote");
    let fungus = reg.interner_mut().intern("Fungus");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_choose_one,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_choose_one(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: modal "choose one" on a triggered ability — TriggeredAbilityDef has no
    //       modal field; the mode selection (put a +1/+1 counter on this / create a
    //       1/1 Saproling) cannot be posted with the demonstrated API.
    Vec::new()
}
