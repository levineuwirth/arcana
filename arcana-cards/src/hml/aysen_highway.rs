//! Aysen Highway — `{3}{W}{W}{W}` enchantment. "White creatures have
//! plainswalk. (They can't be blocked as long as defending player
//! controls a Plains.)"
//!
//! Implementation: ETB-installed filtered keyword grant
//! (`ContinuousEffect::filtered_keyword`) of
//! `KeywordAbility::Landwalk("Plains")` to all white creatures
//! (unscoped — all players'), with
//! `Duration::WhileSourceOnBattlefield`. "Plains" is interned at
//! register time so the effect fn can look it up immutably.

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
    let name = reg.interner_mut().intern("Aysen Highway");
    // Intern "Plains" now so the ETB effect fn can look it up via the
    // immutable interner.
    let _plains = reg.interner_mut().intern("Plains");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_plainswalk,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "white creatures have plainswalk", lasting
/// until this enchantment leaves the battlefield.
fn etb_install_plainswalk(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let plains = reg
        .interner()
        .lookup("Plains")
        .expect("Plains interned at register");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            ObjectFilter::creature().with_colors(ColorSet::white()),
            KeywordAbility::Landwalk(plains),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
