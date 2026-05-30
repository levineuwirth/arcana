//! Thraben Sentry // Thraben Militia — `{3}{W}` Creature — Human Soldier 2/2 (front) /
//! Creature — Human Soldier 5/5 (back). Transform creature.
//!
//! Front face:
//!   Vigilance
//!   Whenever another creature you control dies, you may transform this creature.
//!
//! Back face (Thraben Militia):
//!   Trample
//!
//! GAP: back-face-only triggered abilities not auto-installed on transform.
//! GAP: "you may transform" — optional transform not modeled; fires unconditionally when triggered.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thraben Sentry");
    let human_sub = reg.interner_mut().intern("Human");
    let soldier_sub = reg.interner_mut().intern("Soldier");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Thraben Militia");
    let back_human_sub = reg.interner_mut().intern("Human");
    let back_soldier_sub = reg.interner_mut().intern("Soldier");

    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_human_sub);
    back_subtypes.0.insert(back_soldier_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: whenever another creature you control dies, (you may) transform.
            // GAP: "you may transform" — optional not modeled; triggers unconditionally.
            // GAP: "another creature" — does not filter out self; the creature is already dead when this fires.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: another_creature_dies_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
        // GAP: back-face-only triggered ability not modeled.
    )
}

fn another_creature_dies_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may" — optional transform not modeled.
    vec![Effect::Transform { target: trig.source }]
}
