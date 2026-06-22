//! Spider-Man Noir — `{4}{B}` 4/4 Legendary Creature — Spider Human Hero.
//! Menace.
//! Whenever a creature you control attacks alone, put a +1/+1 counter on it.
//! Then surveil X, where X is the number of counters on it.
//!
//! Decomposition:
//! 1. Keyword line: Menace. (Surveil is the keyword line per Scryfall but here
//!    it is part of the triggered ability's text, not a standalone keyword.)
//! 2. AttacksAlone trigger: add a +1/+1 counter to the sole attacker, then
//!    surveil X where X = the number of counters on that attacker (read at
//!    resolution, AFTER the counter is added — but Surveil's count is taken
//!    from the live state when the ability resolves; we count counters as they
//!    stand at resolution time including the just-added one via a single
//!    Sequence).

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
use arcana_core::effects::KeywordAbility;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spider-Man Noir");
    let spider = reg.interner_mut().intern("Spider");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(human);
    subtypes.0.insert(hero);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::AttacksAlone {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_attacks_alone,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks_alone(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.lone_attacker() else { return Vec::new(); };
    // Counters currently on the attacker BEFORE we add ours; +1 for the one
    // we're about to place gives X for the surveil.
    let existing = state
        .objects
        .get(id)
        .map_or(0u32, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    let x = existing + 1;
    vec![Effect::Sequence(vec![
        Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::Surveil {
            player: trig.controller,
            count: x,
        },
    ])]
}
