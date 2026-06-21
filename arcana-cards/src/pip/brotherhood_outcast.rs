//! Brotherhood Outcast — `{2}{W}` 3/2 white Human Soldier.
//! "When this creature enters, choose one —
//!   • Return target Aura or Equipment card with mana value 3 or less
//!     from your graveyard to the battlefield.
//!   • Put a shield counter on target creature."
//!
//! Abilities:
//! 1. Modal ("choose one") ETB trigger. The demonstrated modal
//!    machinery (ModalSpec / with_mode_effects / dispatch_modal_effect)
//!    is SPELL-ability only; a TriggeredAbilityDef.effect returns a
//!    single Vec<Effect> with no mode-selection hook, so the player's
//!    choice between the two modes cannot be expressed. GAP'd — emitting
//!    one fixed mode would deny the other option (a materially wrong
//!    card). The trigger is still wired (with the mode-1 target
//!    requirement) so the shape is recorded for human routing.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Brotherhood Outcast");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
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

fn etb_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one" ETB. Modal selection is only wired for
    // spell abilities (ModalSpec / with_mode_effects); a triggered
    // ability's effect fn has no mode-choice hook, so neither the
    // graveyard-reanimate mode nor the shield-counter mode can be
    // offered as a choice. Emitting one fixed mode would be wrong.
    Vec::new()
}
