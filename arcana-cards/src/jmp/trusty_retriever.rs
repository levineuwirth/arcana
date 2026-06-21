//! Trusty Retriever — `{3}{W}` 2/3 Dog.
//! "When this creature enters, choose one —
//!  • Put a +1/+1 counter on this creature.
//!  • Return target artifact or enchantment card from your graveyard to
//!    your hand."
//! (A modal CHOICE on a triggered ability — the engine's modal machinery
//! is spell-only (dispatch_modal_effect requires a SpellAbilityDef); there
//! is no choose-one mode primitive for a triggered effect, so the player's
//! mode choice and the second mode's target are GAP'd.)

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trusty Retriever");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
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
    // GAP: "choose one — [+1/+1 counter] or [return target artifact or
    // enchantment from your graveyard]" — modal choice on a triggered
    // ability is not expressible (no choose-one mode primitive for triggers;
    // modal dispatch is spell-only).
    Vec::new()
}
