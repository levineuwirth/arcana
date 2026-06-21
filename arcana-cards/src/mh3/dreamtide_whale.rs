//! Dreamtide Whale — `{2}{U}` 7/5 Whale.
//! Vanishing 2.
//! "Whenever a player casts their second spell each turn, proliferate."
//!
//! Vanishing 2 is a fully-supported parametrized keyword. The proliferate
//! trigger fires on any spell cast; the "their SECOND spell each turn" gate is a
//! fidelity GAP (no per-player spell-ordinal trigger condition), so it fires on
//! each cast instead.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dreamtide Whale");
    let whale = reg.interner_mut().intern("Whale");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(whale);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vanishing(2)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP (fidelity): "their SECOND spell each turn" ordinal gate not
            // expressible; fires on each spell cast.
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: do_proliferate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn do_proliferate(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Proliferate]
}
