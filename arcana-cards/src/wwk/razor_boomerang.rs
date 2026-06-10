//! Razor Boomerang — `{3}` Artifact — Equipment (Mirrodin Besieged).
//! "Equipped creature has \"{T}, Unattach Razor Boomerang: It deals 1
//! damage to any target. Return Razor Boomerang to its owner's hand.\""
//! and "Equip {2}".
//!
//! The Equip half is wired via `with_equip`. GAP: the granted activated
//! ability (with its Unattach cost and self-bounce) is not expressible —
//! only the `attached_pt` P/T static is available for Equipment statics.

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
    let name = reg.interner_mut().intern("Razor Boomerang");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{2}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_static,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_static(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Equipped creature has \"{T}, Unattach Razor Boomerang: It deals
    // 1 damage to any target. Return Razor Boomerang to its owner's
    // hand.\"" — granting an activated ability to the attached creature
    // (and the Unattach cost) is not expressible.
    Vec::new()
}
