//! Killer Cosplay — `{G}` Artifact — Equipment (Unfinity).
//! "Whenever this Equipment becomes attached to a creature, choose a
//! creature card name with an identical mana cost. That creature becomes a
//! copy of the card with the chosen name until this Equipment becomes
//! unattached from it. Equip {3}"
//!
//! `with_equip({3})` wires the Equip activation. GAP: there is no
//! "becomes attached" trigger condition, and "becomes a copy of a chosen
//! card name" has no effect variant — the ETB trigger installs nothing.

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
    let name = reg.interner_mut().intern("Killer Cosplay");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{3}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_noop,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// The copy static is inexpressible; the Equip half is real.
fn etb_noop(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "whenever this Equipment becomes attached" has no trigger
    // condition, and "that creature becomes a copy of the card with the
    // chosen name until unattached" has no effect variant — nothing to
    // install.
    Vec::new()
}
