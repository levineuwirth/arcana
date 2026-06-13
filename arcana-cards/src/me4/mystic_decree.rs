//! Mystic Decree — `{2}{U}{U}` World Enchantment. "All creatures lose
//! flying and islandwalk."
//!
//! The flying removal is wired as a global `filtered_remove_keyword`
//! continuous effect installed on ETB with
//! `Duration::WhileSourceOnBattlefield`. GAP: landwalk loss
//! (islandwalk) is explicitly not expressible — only the flying
//! removal is installed.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mystic Decree");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        supertypes: SupertypeSet(SupertypeSet::WORLD),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_remove_flying,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "all creatures lose flying" anchored to this
/// enchantment's object id, lasting until it leaves the battlefield.
fn etb_install_remove_flying(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "all creatures lose islandwalk" — landwalk loss is not
    // expressible with the current ContinuousEffect builders; only
    // the flying removal is installed.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_remove_keyword(
            trig.source,
            ObjectFilter::creature(),
            KeywordAbility::Flying,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
