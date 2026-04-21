//! Bonesplitter — Mirrodin common. `{1}` artifact — Equipment.
//! "Equipped creature gets +2/+0. Equip `{1}`." The seed's touchstone
//! for CR 702.6 (Equip) and CR 704.5q (illegal-attachment SBA).
//!
//! # Rules references
//!
//! * CR 702.6 — Equip. Activated ability; sorcery speed; target
//!   "creature you control"; resolution attaches this Equipment to
//!   the chosen creature.
//! * CR 702.6a — activating Equip moves the attachment to the new
//!   target, removing any prior attachment.
//! * CR 702.6b — an Equipment attached to a creature that changes
//!   control stays attached (no auto-detach); but see 704.5q.
//! * CR 704.5q — an Equipment attached to an illegal permanent
//!   (non-creature) becomes unattached. Handled by
//!   [`arcana_core::sba::apply_state_based_actions`].
//!
//! # Implementation shape
//!
//! `with_equip({1})` on [`CardDefinition`] appends the canonical Equip
//! activated ability (cost + target-creature-you-control + attach
//! effect). The "+2/+0 to equipped creature" pump is installed as a
//! [`ContinuousEffect::attached_pt`] via an ETB trigger, mirroring
//! [`Glorious Anthem`](arcana_cards::po2::glorious_anthem)'s pattern.
//! The layer's `applies_to` reads `attached_to` dynamically so the
//! pump follows the equipment.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Bonesplitter");
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
            .with_equip(ManaCost::parse("{1}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_attached_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
            }),
    )
}

/// ETB trigger: install a layer-7c "attached creature gets +2/+0"
/// continuous effect anchored to this Equipment. The effect's
/// `applies_to` dereferences `attached_to` every time characteristics
/// are computed, so the pump automatically follows each re-equip and
/// expires when the Equipment leaves the battlefield.
fn etb_install_attached_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            2,
            0,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
