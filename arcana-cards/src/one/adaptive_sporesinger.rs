//! Adaptive Sporesinger — `{2}{G}` 2/2 Phyrexian Druid with Vigilance.
//!
//! Oracle text:
//! * Vigilance — keyword, base characteristic.
//! * "When this creature enters, choose one —
//!     • Target creature gets +2/+2 and gains vigilance until end of turn.
//!     • Proliferate."
//!   An ETB MODAL ("choose one") trigger. The demonstrated modal
//!   machinery (`ModalSpec` / `with_mode_effects` / `dispatch_modal_effect`)
//!   is SPELL-ability only; a `TriggeredAbilityDef.effect` returns a
//!   single `Vec<Effect>` with no mode-selection hook, so the player's
//!   choice between the two modes cannot be expressed here.
//!   GAP: modal ETB choice — emitting one fixed mode would be a wrong
//!   card (it would deny the other option), so the trigger body returns
//!   `Vec::new()`. The trigger is still wired (with the mode-0 target
//!   requirement) so the shape is recorded for human routing.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adaptive Sporesinger");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn etb_modal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: modal "choose one" ETB. Modal selection is only wired for
    // spell abilities (ModalSpec / with_mode_effects); a triggered
    // ability's effect fn has no mode-choice hook, so neither the
    // mode-0 pump+vigilance nor the mode-1 Proliferate can be offered
    // as a choice. Emitting one fixed mode would be materially wrong.
    Vec::new()
}
