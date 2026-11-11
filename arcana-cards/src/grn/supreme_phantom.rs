//! Supreme Phantom — `{1}{U}` Spirit, 1/3, flying.
//! Other Spirits you control get +1/+1 (A1.5bq).
//! A1.5br still owns replacing this ETB installer with an immediate static.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::zones::Zone;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Supreme Phantom");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1, trigger_condition: TriggerCondition::SelfEntersBattlefield,
        intervening_if: None, effect: install_anthem,
        trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    }))
}

fn install_anthem(_: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").expect("registered Spirit");
    vec![Effect::InstallContinuousEffect { effect: ContinuousEffect::filtered_pump(
        trig.source,
        ObjectFilter::creature().with_subtype_sym(spirit)
            .controlled_by(ControllerConstraint::You).excluding_source(),
        1, 1, Duration::WhileSourceOnBattlefield,
    ) }]
}
