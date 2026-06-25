//! Tormentor's Trident — `{2}` artifact — Equipment (Magic Origins,
//! 2015). "Equipped creature gets +3/+0 and attacks each combat if
//! able." and "Equip {3}".
//! The +3/+0 static is installed as an attached-P/T continuous effect.
//! GAP: "equipped creature ... attacks each combat if able" needs an
//! attachment-following `attached_must_attack` continuous effect — the
//! `must_attack(target)` primitive takes a static ObjectId, and an
//! Equipment enters unattached and can re-equip, so no fixed target is
//! known at ETB.

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
    let name = reg.interner_mut().intern("Tormentor's Trident");
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
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_attached_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install the layer-7c "equipped creature gets +3/+0"
/// continuous effect anchored to this Equipment.
fn etb_install_attached_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "equipped creature ... attacks each combat if able" — needs an
    // attachment-following attached_must_attack (must_attack takes a static
    // target id; an Equipment has no fixed equipped creature at ETB).
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            3,
            0,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
