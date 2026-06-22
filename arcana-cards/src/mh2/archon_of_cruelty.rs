//! Archon of Cruelty — `{6}{B}{B}` 6/6 Archon with Flying.
//!
//! Flying
//! Whenever this creature enters or attacks, target opponent sacrifices a
//! creature or planeswalker of their choice, discards a card, and loses 3
//! life. You draw a card and gain 3 life.
//!
//! Decomposed as: a keyword line (Flying) plus two triggers (enters,
//! attacks) sharing one resolver. The resolver targets an opponent and
//! makes them sacrifice a creature-or-planeswalker, discard a card, and
//! lose 3 life, then draws a card and gains 3 life for you.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archon of Cruelty");
    let archon = reg.interner_mut().intern("Archon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(archon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: cruelty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target_opponent()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: cruelty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target_opponent()],
            }),
    )
}

fn target_opponent() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Player,
        count: TargetCount::Exactly(1),
        controller: Some(ControllerConstraint::Opponent),
    }
}

fn cruelty(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(opp) = target else {
        return Vec::new();
    };
    let opp = *opp;
    let creature_or_pw =
        ObjectFilter::permanent().with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER));
    vec![Effect::Sequence(vec![
        Effect::Sacrifice {
            player: opp,
            filter: creature_or_pw,
            count: 1,
        },
        Effect::Discard {
            player: opp,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::LoseLife { player: opp, amount: 3 },
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::GainLife { player: trig.controller, amount: 3 },
    ])]
}
