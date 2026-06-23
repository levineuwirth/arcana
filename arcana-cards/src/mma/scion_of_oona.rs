//! Scion of Oona — `{2}{U}` 1/1 Faerie Soldier.
//!
//! Flash
//! Flying
//! Other Faerie creatures you control get +1/+1.
//! Other Faeries you control have shroud.
//!
//! The keyword line (Flash, Flying) is base characteristics. The two static
//! lord abilities are installed via a `SelfEntersBattlefield` trigger as two
//! continuous effects filtered to Faeries you control: a `filtered_pump`
//! (+1/+1) and a `filtered_keyword` (Shroud). Per the documented minor
//! fidelity gap, the "OTHER" exclusion of the source itself is not modeled —
//! `filtered_pump`/`filtered_keyword` match base characteristics, so Scion of
//! Oona (a Faerie) is also affected by its own buff/grant.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scion of Oona");
    let faerie = reg.interner_mut().intern("Faerie");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_faerie_statics,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Install the two static lord effects scoped to Faeries you control:
/// "+1/+1" and "have shroud". Both last while Scion of Oona is on the
/// battlefield.
fn install_faerie_statics(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let faeries = script::subtype_filter(reg, "Faerie").controlled_by(ControllerConstraint::You);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                faeries.clone(),
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                faeries,
                KeywordAbility::Shroud,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
