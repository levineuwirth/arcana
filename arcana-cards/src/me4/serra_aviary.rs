//! Serra Aviary — `{3}{W}` World enchantment. "Creatures with flying
//! get +1/+1."
//!
//! Implementation: ETB-installed filtered pump
//! (`ContinuousEffect::filtered_pump`) over all creatures with flying
//! (unscoped — no controller constraint), with
//! `Duration::WhileSourceOnBattlefield`.
//!
//! Note: the World supertype is recorded in `supertypes`; the world
//! rule (CR 704.5m) itself is a state-based action outside this
//! card's scope.

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
    let name = reg.interner_mut().intern("Serra Aviary");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                effect: etb_install_flyer_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "creatures with flying get +1/+1" (all
/// players' creatures), lasting until this enchantment leaves the
/// battlefield.
fn etb_install_flyer_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
