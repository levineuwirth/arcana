//! Spider-Suit — `{1}` artifact — Equipment (Marvel's Spider-Man).
//! "Equipped creature gets +2/+2 and is a Spider Hero in addition to
//! its other types. Equip {3}"
//!
//! The Equip activation is wired via `with_equip`; the +2/+2 static
//! installs `ContinuousEffect::attached_pt`, and the "is a Spider Hero
//! in addition to its other types" half installs
//! `ContinuousEffect::attached_subtypes` (Layer 4, additive).

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
    let name = reg.interner_mut().intern("Spider-Suit");
    let equipment = reg.interner_mut().intern("Equipment");
    reg.interner_mut().intern("Spider");
    reg.interner_mut().intern("Hero");
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

fn etb_install_attached_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "is a Spider Hero in addition to its other types" — attached
    // subtype grant (Layer 4, additive) following the attachment.
    let mut subs = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Spider") {
        subs.0.insert(s);
    }
    if let Some(s) = reg.interner().lookup("Hero") {
        subs.0.insert(s);
    }
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                2,
                2,
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
