//! Craig Boone, Novac Guard — `{1}{R}{W}` 3/3 Legendary Human Soldier,
//! reach and lifelink. "One for My Baby — Whenever you attack with two
//! or more creatures, put two quest counters on Craig Boone. When you
//! do, Craig Boone deals damage equal to the number of quest counters on
//! it to up to one target creature unless that creature's controller has
//! Craig Boone deal that much damage to them."
//!
//! Modeled: put two quest counters on the source. GAP'd: the "two or
//! more creatures" attacker-count gate (no such trigger condition / no
//! attacker-count predicate) — we fire on any attack — and the reflexive
//! "When you do, … deals damage … unless that creature's controller …"
//! redirect-or-damage clause (no reflexive/unless-redirect primitive).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Craig Boone, Novac Guard");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "attack with two or more creatures" — no attacker-count trigger;
            // approximated with "a creature you control attacks".
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: add_quest_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn add_quest_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reflexive "When you do, Craig Boone deals damage equal to the number
    // of quest counters on it to up to one target creature unless that
    // creature's controller has Craig Boone deal that much damage to them."
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Quest,
        count: 2,
    }]
}
