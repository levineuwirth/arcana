//! Angelic Armaments — `{3}` artifact — Equipment.
//! "Equipped creature gets +2/+2, has flying, and is a white Angel in
//! addition to its other colors and types. Equip {4}"
//! The +2/+2 static installs via attached_pt, the flying grant via
//! attached_keyword, and the white-Angel addition via attached_colors
//! (Layer 5, additive) + attached_subtypes (Layer 4, additive).

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
    let name = reg.interner_mut().intern("Angelic Armaments");
    let equipment = reg.interner_mut().intern("Equipment");
    reg.interner_mut().intern("Angel");
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
            .with_equip(ManaCost::parse("{4}").expect("valid cost"))
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

/// ETB trigger: install the layer "equipped creature gets +2/+2"
/// continuous effect anchored to this Equipment.
fn etb_install_attached_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "equipped creature gets +2/+2 and has flying" — attached P/T plus
    // an attached keyword grant; "is a white Angel in addition to its
    // other colors and types" — attached color (Layer 5, additive) and
    // subtype (Layer 4, additive) grants, all following the attachment.
    let mut subs = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Angel") {
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
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_colors(
                trig.source,
                ColorSet::white(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
