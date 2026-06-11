//! Illusionist's Bracers — `{2}` artifact — Equipment.
//! "Whenever an ability of equipped creature is activated, if it isn't
//! a mana ability, copy that ability. You may choose new targets for
//! the copy. Equip {3}"
//!
//! The Equip half is real; the ability-copy trigger is a documented gap
//! (see notes).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Illusionist's Bracers");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{3}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever an ability of equipped
                // creature is activated" has no ability-activated
                // trigger condition; SelfEntersBattlefield is a
                // placeholder anchor and the resolution is a no-op.
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: copy_activated_ability,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…if it isn't a mana ability, copy that ability. You may choose new
/// targets for the copy."
fn copy_activated_ability(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no trigger condition observes ability activations, no
    // accessor exposes the activated ability's stack entry, and there
    // is no copy-ability effect (CopySpell takes a spell's stack id
    // which is unavailable here). Resolution is a documented no-op.
    Vec::new()
}
