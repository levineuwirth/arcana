//! Raven Wings — `{2}` artifact — Equipment (Kaldheim).
//! "Equipped creature gets +1/+0, has flying, and is a Bird in
//! addition to its other types. Equip {2}."
//!
//! The +1/+0 static is installed as an attached-P/T continuous effect
//! via the ETB trigger; Equip {2} via `with_equip`.
//!
//! The flying grant installs as an attached-keyword continuous effect;
//! the "is a Bird in addition to its other types" half installs as an
//! attached-subtype continuous effect (Layer 4, additive).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Raven Wings");
    let equipment = reg.interner_mut().intern("Equipment");
    reg.interner_mut().intern("Bird");
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
            .with_equip(ManaCost::parse("{2}").expect("valid cost"))
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
/// continuous effect, the "has flying" keyword grant, and the layer-4
/// "is a Bird in addition to its other types" subtype grant, all
/// anchored to this Equipment.
fn etb_install_attached_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut subs = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Bird") {
        subs.0.insert(s);
    }
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                1,
                0,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Flying,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                subs,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
