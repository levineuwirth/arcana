//! Bereaved Survivor // Dauntless Avenger
//!
//! Front: {2}{W} Creature — Human Peasant 2/1.
//! When another creature you control dies, transform this creature.
//!
//! Back: Dauntless Avenger — Creature — Human Soldier.
//! Whenever this creature attacks, return target creature card with mana value 2
//! or less from your graveyard to the battlefield tapped and attacking.
//!
//! GAP: Back face "return target creature card with mana value 2 or less from
//!   your graveyard to the battlefield tapped and attacking" — the
//!   ReturnFromGraveyardToBattlefield effect does not model entering tapped and
//!   attacking; the "tapped and attacking" rider is omitted (card enters normally).
//! GAP: back-face-only triggered ability (attack trigger on Dauntless Avenger) not
//!   modeled automatically. The attack trigger is authored here on the CardDefinition
//!   but will fire on both faces (engine debt).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
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
    let name = reg.interner_mut().intern("Bereaved Survivor");
    let human_sub = reg.interner_mut().intern("Human");
    let peasant_sub = reg.interner_mut().intern("Peasant");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human_sub);
    front_subtypes.0.insert(peasant_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Dauntless Avenger");
    let human_back = reg.interner_mut().intern("Human");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_back);
    back_subtypes.0.insert(soldier_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face trigger: when another creature you control dies, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: creature_dies_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Back face attack trigger: when this creature attacks, return a creature
            // card with mana value 2 or less from your graveyard to the battlefield.
            // GAP: engine lacks face-gating on triggered abilities; fires on both faces.
            // GAP: "tapped and attacking" rider not modeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0), // 0 = any player (engine ignores player index in Graveyard match)
                        filter: ObjectFilter::creature().with_max_cmc(2),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

/// Front face: another creature you control died — transform.
fn creature_dies_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

/// Back face attack trigger: return a creature card with mv ≤ 2 from your
/// graveyard to the battlefield.
/// GAP: enters tapped and attacking not modeled.
fn attack_reanimate(
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
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
