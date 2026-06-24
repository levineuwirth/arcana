//! Traproot Kami — `{G}` 0/* Creature — Spirit.
//! Defender, Reach.
//! Traproot Kami's toughness is equal to the number of Forests on the battlefield.
//!
//! Asymmetric subtype CDA (power fixed 0, toughness = the count): wired at
//! Layer 7a via `self_pt_from_match_asym` over a Forest-subtype filter (the
//! filter is built in the ETB fn, which has the interner to name "Forest").

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Traproot Kami");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_triggered_ability(
        TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        },
    ))
}

/// Toughness = Forests on the battlefield (all controllers); power fixed 0.
fn install_cda(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let forests = script::subtype_filter(reg, "Forest");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match_asym(
            trig.source, forests, /*count_is_power=*/ false, /*other_fixed=*/ 0,
            Duration::WhileSourceOnBattlefield),
    }]
}
