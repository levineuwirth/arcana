//! Hidden Path — `{2}{G}{G}{G}{G}` enchantment. "Green creatures have
//! forestwalk."
//!
//! Implementation: an ETB trigger installs a
//! [`ContinuousEffect::filtered_keyword`] granting
//! `KeywordAbility::Landwalk("Forest")` to all green creatures
//! (unscoped — every player's), with
//! [`Duration::WhileSourceOnBattlefield`].

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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hidden Path");
    let _forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
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

/// ETB trigger: install "green creatures have forestwalk", anchored to
/// this enchantment and lasting until it leaves the battlefield.
fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest = reg
        .interner()
        .lookup("Forest")
        .expect("Forest interned during register()");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            ObjectFilter::creature().with_colors(ColorSet::green()),
            KeywordAbility::Landwalk(forest),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
