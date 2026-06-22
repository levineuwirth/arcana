//! Battlegrace Angel — `{3}{W}{W}` 4/4 Angel with Flying and Exalted.
//!
//! * Flying, Exalted (keywords).
//! * "Whenever a creature you control attacks alone, it gains lifelink
//!   until end of turn." — an `AttacksAlone` trigger (filtered to a
//!   creature you control) that grants lifelink to the sole attacker.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Battlegrace Angel");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Exalted],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::AttacksAlone {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: grant_lifelink_to_lone_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn grant_lifelink_to_lone_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.lone_attacker() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: id,
        keyword: KeywordAbility::Lifelink,
        duration: Duration::EndOfTurn,
    }]
}
