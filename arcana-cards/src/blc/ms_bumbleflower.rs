//! Ms. Bumbleflower — `{1}{G}{W}{U}` 1/5 Legendary Rabbit Citizen with Vigilance.
//! "Whenever you cast a spell, target opponent draws a card. Put a +1/+1 counter
//!  on target creature. It gains flying until end of turn. If this is the second
//!  time this ability has resolved this turn, you draw two cards."
//!
//! Vigilance is a base keyword. The cast trigger draws for the target opponent,
//! adds a +1/+1 counter to the target creature, and grants it flying. The
//! "second time this ability has resolved this turn → you draw two" rider has no
//! per-ability-resolution-count accessor — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ms. Bumbleflower");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: on_cast,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
        }),
    )
}

fn on_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut player_target = None;
    let mut creature_target = None;
    for t in &trig.targets.targets {
        match t {
            TargetChoice::Player(p) => player_target = Some(*p),
            TargetChoice::Object(id) => creature_target = Some(*id),
            TargetChoice::ObjectOrPlayer(_) => {}
        }
    }
    let mut effects = Vec::new();
    if let Some(p) = player_target {
        effects.push(Effect::DrawCards { player: p, count: 1 });
    }
    if let Some(id) = creature_target {
        effects.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        });
    }
    // GAP: "if this is the second time this ability has resolved this turn, you
    // draw two cards" — no per-ability-resolution-count accessor.
    effects
}
