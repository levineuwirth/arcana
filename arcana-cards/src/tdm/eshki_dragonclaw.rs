//! Eshki Dragonclaw — `{1}{G}{U}{R}` 4/4 Legendary Human Warrior with
//! Vigilance, Trample, and Ward {1}.
//! "At the beginning of combat on your turn, if you've cast both a
//! creature spell and a noncreature spell this turn, draw a card and
//! put two +1/+1 counters on Eshki Dragonclaw."
//!
//! Vigilance + Trample + Ward {1} are base keywords. The combat-begin
//! trigger has an intervening-if gating on having cast both a creature
//! and a noncreature spell this turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eshki Dragonclaw");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_cast_creature_and_noncreature),
                effect: draw_and_grow,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_cast_creature_and_noncreature(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    let creature = ObjectFilter::new().with_types(TypeLine::CREATURE.into());
    let noncreature = ObjectFilter::new().without_types(TypeLine::CREATURE.into());
    script::spells_cast_this_turn(s, &creature, you) >= 1
        && script::spells_cast_this_turn(s, &noncreature, you) >= 1
}

fn draw_and_grow(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        },
    ]
}
