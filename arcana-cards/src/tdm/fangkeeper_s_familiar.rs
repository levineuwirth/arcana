//! Fangkeeper's Familiar — `{1}{B}{G}{U}` 3/3 Sultai Snake.
//!
//! Oracle:
//! * Flash
//! * When this creature enters, choose one —
//!   • You gain 3 life and surveil 3.
//!   • Destroy target enchantment.
//!   • Counter target creature spell.
//!
//! Flash is a base keyword. The ETB is a MODAL ("choose one") triggered
//! ability; `TriggeredAbilityDef` has no modal field (modal is only modeled
//! for spell abilities), and the trigger picks among targeted modes whose
//! targets must be declared up front. There is no way to express a modal
//! trigger with this surface — emitting any single mode would mis-state the
//! card — so the ETB is GAP'd in full.

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
    let name = reg.interner_mut().intern("Fangkeeper's Familiar");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_modal_choice,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_modal_choice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal ("choose one") triggered ability is not expressible — no
    // modal field on TriggeredAbilityDef; the modes are gain-life+surveil,
    // destroy target enchantment, and counter target creature spell.
    Vec::new()
}
