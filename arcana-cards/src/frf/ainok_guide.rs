//! Ainok Guide — `{1}{G}` 1/1 Dog Scout.
//! "When this creature enters, choose one —
//!  • Put a +1/+1 counter on this creature.
//!  • Search your library for a basic land card, reveal it, then shuffle
//!    and put that card on top."
//!
//! The ETB ability is MODAL ("choose one"), but the documented modal
//! machinery (ModalSpec / mode_effects) applies only to SPELLS, not
//! triggered abilities. There is no modal-trigger primitive, so the
//! per-mode choice cannot be expressed — the trigger effect body is
//! GAP'd while the ETB trigger shape is kept.

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
    let name = reg.interner_mut().intern("Ainok Guide");
    let dog = reg.interner_mut().intern("Dog");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
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
    // GAP: modal "choose one — +1/+1 counter on self / search for a basic
    // land to top" — modal selection is spell-only; no modal-trigger primitive.
    Vec::new()
}
