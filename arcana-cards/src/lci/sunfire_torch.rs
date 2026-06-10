//! Sunfire Torch — `{R}` artifact — Equipment (Duskmourn: House of
//! Horror, 2024). "Equipped creature gets +1/+0 and has \"Whenever
//! this creature attacks, you may sacrifice Sunfire Torch. When you
//! do, this creature deals 2 damage to any target.\" Equip {1}"
//!
//! The Equip activation and the +1/+0 attached pump are wired; the
//! granted attack trigger (with its reflexive sacrifice and damage) is
//! GAP'd — the Equipment static surface has no attached
//! triggered-ability grant.

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
    let name = reg.interner_mut().intern("Sunfire Torch");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install the layer-7c "attached creature gets +1/+0"
/// continuous effect anchored to this Equipment.
fn etb_install_attached_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "has \"Whenever this creature attacks, you may sacrifice
    // Sunfire Torch. When you do, this creature deals 2 damage to any
    // target.\"" — granting a triggered ability (with a reflexive
    // sacrifice rider) to the dynamically-attached creature is not
    // expressible in the Equipment static surface; only the P/T pump is
    // installed.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            1,
            0,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
