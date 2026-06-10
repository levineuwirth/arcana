//! Bloodthirsty Blade — `{2}` artifact — Equipment.
//! "Equipped creature gets +2/+0 and is goaded. {1}: Attach this Equipment
//! to target creature an opponent controls. Activate only as a sorcery."
//! The +2/+0 static is installed as a layer-7c `attached_pt` continuous
//! effect via an ETB trigger. There is NO printed `Equip {N}` line — the
//! card's bespoke "{1}: Attach … to target creature an opponent controls"
//! activation is not the canonical Equip shape (`with_equip` targets a
//! creature YOU control), so it is a documented gap, as is the continuous
//! "is goaded" static.

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
    let name = reg.interner_mut().intern("Bloodthirsty Blade");
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
    // GAP: '{1}: Attach this Equipment to target creature an opponent
    // controls. Activate only as a sorcery.' — with_equip is the only
    // attachment path and it targets a creature YOU control; the
    // opponent-targeting attach activation is not expressible here.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_attached_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// ETB trigger: install the layer-7c "attached creature gets +2/+0"
/// continuous effect anchored to this Equipment.
fn etb_install_attached_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'equipped creature is goaded' — attached_pt covers P/T only;
    // a continuous attached-goad static is not expressible.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            2,
            0,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
