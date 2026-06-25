//! Devoted Grafkeeper // Departed Soulkeeper — `{W}{U}` Human Peasant 2/1 (front).
//!
//! Front:
//!   When this creature enters, mill two cards.
//!   Whenever you cast a spell from your graveyard, tap target creature you don't control.
//!   Disturb {1}{W}{U}
//!
//! Back (Departed Soulkeeper):
//!   Flying
//!   This creature can block only creatures with flying. (GAP — not expressible)
//!   If Departed Soulkeeper would be put into a graveyard from anywhere, exile it instead.
//!   (GAP — replacement effect not modeled)
//!
//! # GAPs
//! - Disturb (cast from graveyard transformed) not modeled as an alternate cast — GAP.
//! - Back "can block only creatures with flying" — restriction on blocking is not
//!   expressible with the current Effect API.
//! - Back "If ... would be put into a graveyard ... exile it instead" — replacement effect
//!   not modeled.
//! - "Whenever you cast a spell from your graveyard" — wired via SpellCastFromZone.
//! - Back-face-only triggered abilities not auto-installed on transform.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetFilter, TargetCount, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Devoted Grafkeeper");
    let human_sub = reg.interner_mut().intern("Human");
    let peasant_sub = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(peasant_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Departed Soulkeeper");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Flying],
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            // GAP: "can block only creatures with flying" — blocking restriction not modeled
            // GAP: "if would be put into graveyard, exile instead" — replacement not modeled
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // When this creature enters, mill two cards.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: etb_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Whenever you cast a spell from your graveyard, tap target creature you don't
            // control.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCastFromZone {
                    filter: None,
                    caster: ControllerConstraint::You,
                    from_zone: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: tap_target_opponent_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_mill(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Mill { player: trig.controller, count: 2 }]
}

fn tap_target_opponent_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Tap { target: *id }]
}
