//! Chandra's Pyreling — `{1}{R}` 1/3 red Elemental Lizard.
//! "Whenever a source you control deals noncombat damage to an
//! opponent, this creature gets +1/+0 and gains double strike until
//! end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra's Pyreling");
    let elemental = reg.interner_mut().intern("Elemental");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — DamageDealt cannot express
                // "noncombat damage only" (combat_only: false matches
                // both combat and noncombat damage). Also, target_filter
                // cannot constrain to an opponent specifically;
                // TargetFilter::Player matches any player including
                // the controller.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: pump_and_double_strike,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// +1/+0 and gain double strike until end of turn — granted to this
/// creature (the trigger's source).
fn pump_and_double_strike(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::DoubleStrike],
    }]
}
