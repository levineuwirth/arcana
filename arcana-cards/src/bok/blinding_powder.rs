//! Blinding Powder — `{1}` artifact — Equipment.
//! "Equipped creature has 'Unattach Blinding Powder: Prevent all
//! combat damage that would be dealt to this creature this turn.'
//! Equip {2}." `with_equip` wires the canonical Equip activation.
//!
//! GAP: "Equipped creature has 'Unattach Blinding Powder: …'" —
//! granting an activated ability (with an unattach cost, which has
//! no `ActivationCost` field at all) to the attached creature is not
//! expressible; the ETB effect fn installs nothing.

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
    let name = reg.interner_mut().intern("Blinding Powder");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
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
                effect: etb_noop,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// GAP: "Equipped creature has 'Unattach Blinding Powder: Prevent
/// all combat damage that would be dealt to this creature this
/// turn.'" — no attached ability grant and no unattach cost exist;
/// nothing is installed.
fn etb_noop(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
