//! Krovikan Mist — `{1}{U}` */* Creature — Illusion.
//! "Flying
//!  Krovikan Mist's power and toughness are each equal to the number of
//!  Illusions on the battlefield."
//!
//! The */* CDA is wired via a self-CDA installed on ETB
//! (ContinuousEffect::self_pt_from_match at Layer 7a): both P/T equal the
//! number of Illusions on the battlefield (all controllers — no
//! `controlled_by` constraint).

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
    let name = reg.interner_mut().intern("Krovikan Mist");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Layer 7a self-CDA: P/T each equal to the number of Illusions on the
/// battlefield (all controllers — no `controlled_by` constraint).
fn install_cda(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Illusion");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
