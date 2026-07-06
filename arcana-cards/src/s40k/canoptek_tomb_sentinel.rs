//! Canoptek Tomb Sentinel — `{4}` 4/3 Artifact Creature — Insect.
//! Vigilance.
//! Exile Cannon — When this creature enters from a graveyard, exile up to one
//! target nonland permanent.
//! Unearth {7}.

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
    let name = reg.interner_mut().intern("Canoptek Tomb Sentinel");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "enters from a graveyard" is gated inside the effect via
            // PendingTrigger::entered_from_zone() (a plain SelfEntersBattlefield;
            // the exile no-ops on a normal cast). Minor wart: the target is still
            // prompted on a hand cast. Unearth {7} is not an expressible cost and
            // is omitted (the ability still fires via other reanimation).
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: exile_nonland,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn exile_nonland(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "When this enters FROM A GRAVEYARD" (via Unearth/reanimation) — a normally
    // cast Sentinel enters from the stack and exiles nothing.
    if !matches!(trig.entered_from_zone(), Some(Zone::Graveyard(_))) {
        return Vec::new();
    }
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use arcana_core::events::{GameEvent, MoveCause};
    use arcana_core::triggers::PendingTrigger;

    fn trig_from(from: Zone) -> PendingTrigger {
        let mut targets = arcana_core::targets::TargetSelection::default();
        targets.targets.push(TargetChoice::Object(9));
        PendingTrigger {
            source: 5,
            trigger_id: 1,
            controller: 0,
            trigger_event: GameEvent::ZoneChange {
                object_id: 4,
                from,
                to: Zone::Battlefield,
                new_id: 5,
                cause: MoveCause::StateBasedAction,
            },
            targets,
            effect_override: None,
        }
    }

    #[test]
    fn exiles_only_when_entering_from_a_graveyard() {
        let reg = CardRegistry::new();
        let s = GameState::new(2, 0);
        // Reanimated/Unearthed (from graveyard): exile the chosen target.
        match exile_nonland(&s, &trig_from(Zone::Graveyard(0)), &reg).as_slice() {
            [Effect::ExilePermanent { target: 9 }] => {}
            other => panic!("from graveyard should exile the target, got {other:?}"),
        }
        // Normally cast (from the stack): the exile no-ops.
        assert!(exile_nonland(&s, &trig_from(Zone::Stack), &reg).is_empty(),
            "a hand cast enters from the stack → no exile");
    }
}
