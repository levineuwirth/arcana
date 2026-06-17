//! Shark Shredder, Killer Clone — `{2}{B}{B}` 4/4 Legendary Shark
//! Octopus Ninja with First strike.
//! "Sneak {3}{B}{B}
//!  First strike
//!  Whenever Shark Shredder deals combat damage to a player, put up to
//!  one target creature card from that player's graveyard onto the
//!  battlefield under your control. It enters tapped and attacking that
//!  player."
//!
//! Sneak is not an expressible keyword (alternative cost) — GAP. The
//! combat-damage trigger is wired; targeting any graveyard (not strictly
//! the damaged player's) and the exact attacked player are fidelity gaps.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shark Shredder, Killer Clone");
    let shark = reg.interner_mut().intern("Shark");
    let octopus = reg.interner_mut().intern("Octopus");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shark);
    subtypes.0.insert(octopus);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: arcana_core::types::SupertypeSet(
            arcana_core::types::SupertypeSet::LEGENDARY,
        ),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };
    // GAP: "Sneak {3}{B}{B}" — alternative-cost keyword not expressible.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: reanimate_attacking,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn reanimate_attacking(
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
    vec![Effect::PutOntoBattlefieldTappedAttacking {
        target: *id,
        controller: trig.controller,
    }]
}
