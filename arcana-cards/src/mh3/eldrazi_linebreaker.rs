//! Eldrazi Linebreaker — `{1}{C}{R}` 3/3 Eldrazi with Devoid (colorless) and
//! Trample. At the beginning of combat on your turn, target creature you control
//! gains haste and gets +X/+0 until end of turn, where X is the number of Eldrazi
//! you control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eldrazi Linebreaker");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    // Devoid → the card has no color.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{C}{R}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: haste_and_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn haste_and_pump(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Eldrazi").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::Pump {
        target: *id,
        power: x as i32,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Haste],
    }]
}
