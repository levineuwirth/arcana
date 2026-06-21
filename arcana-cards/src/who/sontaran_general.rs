//! Sontaran General — `{4}{R}` 5/5 Alien Soldier with Trample, Haste.
//!
//! Oracle:
//! * Trample, haste — keywords. (Scryfall also lists `Goad` and
//!   `Battalion`; neither is a supported `KeywordAbility` variant and both
//!   describe the triggered ability below, so they are not emitted as
//!   keywords.)
//! * Battalion — Whenever this creature and at least two other creatures
//!   attack, for each opponent, goad up to one target creature that player
//!   controls. Those creatures can't block this turn. — modeled as a
//!   `SelfAttacks` trigger that goads up to one targeted creature an
//!   opponent controls and forbids it from blocking. GAPs: the Battalion
//!   gate ("at least two other creatures attack") is not expressible as an
//!   intervening-if over attacker counts, and the per-opponent fan-out is
//!   collapsed to a single up-to-one target.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sontaran General");
    let alien = reg.interner_mut().intern("Alien");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — Battalion ("this creature and at least two
            // other creatures attack"). No attacker-count condition exists;
            // approximated as a plain SelfAttacks trigger.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: goad_and_forbid_block,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "for each opponent" fan-out collapsed to a single up-to-one
            // target creature an opponent controls.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn goad_and_forbid_block(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::Goad {
            target: *id,
            goader: trig.controller,
            duration: Duration::EndOfTurn,
        },
        Effect::ForbidBlocking {
            target: *id,
            duration: Duration::EndOfTurn,
        },
    ]
}
