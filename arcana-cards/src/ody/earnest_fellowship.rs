//! Earnest Fellowship — `{1}{W}` enchantment.
//! "Each creature has protection from its colors."
//!
//! Standalone static. "Protection from its own colors" is symmetric and
//! decomposes cleanly per color: for each of the five colors X, every
//! creature that IS color X gains protection from X. We install one
//! `filtered_keyword` per color, filtering `creature().with_colors(X)`
//! (which requires color X be present — `with_colors` is a superset match)
//! and granting `ProtectionQuality::Color(X)`. A creature that is, say,
//! white-and-blue thus matches both the white and blue filters and gains
//! protection from white and from blue. All five last while this
//! enchantment remains on the battlefield.

use arcana_core::effects::{Effect, KeywordAbility, ProtectionQuality};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, Color, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Earnest Fellowship");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_install,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // One install per color: color-X creatures get protection from X.
    let per_color = [
        (ColorSet::white(), Color::White),
        (ColorSet::blue(), Color::Blue),
        (ColorSet::black(), Color::Black),
        (ColorSet::red(), Color::Red),
        (ColorSet::green(), Color::Green),
    ];
    per_color
        .into_iter()
        .map(|(set, color)| Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                ObjectFilter::creature().with_colors(set),
                KeywordAbility::Protection(ProtectionQuality::Color(color)),
                Duration::WhileSourceOnBattlefield,
            ),
        })
        .collect()
}
