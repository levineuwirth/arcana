//! Charming Scoundrel — `{1}{R}` 1/1 Human Rogue with Haste.
//! "When this creature enters, choose one —
//!  • Discard a card, then draw a card.
//!  • Create a Treasure token.
//!  • Create a Wicked Role token attached to target creature you control."
//!
//! Haste is a base keyword (the Role token / Treasure entries are
//! mechanic markers, not `KeywordAbility` variants). The ETB ability is
//! a modal "choose one" — but the modal dispatch machinery
//! (ModalSpec / dispatch_modal_effect) is only available on SPELL
//! abilities, not triggered abilities, so the chooser can't be wired.
//! The ETB trigger is kept as a shell with its effect GAP'd.

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
    let name = reg.interner_mut().intern("Charming Scoundrel");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
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

fn etb_choose_one(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ETB modal "choose one" — modal dispatch (ModalSpec /
    // dispatch_modal_effect) is only available on spell abilities, not
    // triggered abilities, so the player's mode choice cannot be wired.
    // Effect omitted.
    Vec::new()
}
