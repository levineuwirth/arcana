//! Miles Morales // Ultimate Spider-Man — MDFC.
//! Front: `{1}{G}` Legendary Creature — Human Citizen Hero 1/2 (green).
//!   When Miles Morales enters, put a +1/+1 counter on each of up to two target creatures.
//!   {3}{R}{G}{W}: Transform Miles Morales. Activate only as a sorcery.
//!
//! Back: Ultimate Spider-Man — `{3}{R}{G}{W}` Legendary Creature — Spider Human Hero 6/6 (multicolor R/G/W).
//!   First strike, haste
//!   Camouflage — {2}: Put a +1/+1 counter on Ultimate Spider-Man. He gains hexproof and becomes colorless until end of turn.
//!   Whenever you attack, double the number of each kind of counter on each Spider and legendary creature you control.
//!     (GAP: "double counters" is not expressible with the current Effect catalog.)
//!
//! This is a transforming DFC: {3}{R}{G}{W} sorcery-speed Transform is wired as a
//!   front-face activated ability (face 0). The back-face Camouflage activated ability
//!   ({2}: +1/+1 counter + gains hexproof + becomes colorless until EOT) is fully wired,
//!   gated to face 1.
//! GAP: back-face "Whenever you attack, double the number of each kind of counter …" — no
//!   double-counters Effect variant in the catalog; the attack trigger is left unwired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Miles Morales");
    let human_sub = reg.interner_mut().intern("Human");
    let citizen_sub = reg.interner_mut().intern("Citizen");
    let hero_sub = reg.interner_mut().intern("Hero");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(citizen_sub);
    subtypes.0.insert(hero_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ultimate Spider-Man");
    let back_spider_sub = reg.interner_mut().intern("Spider");
    let back_human_sub = reg.interner_mut().intern("Human");
    let back_hero_sub = reg.interner_mut().intern("Hero");

    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_spider_sub);
    back_subtypes.0.insert(back_human_sub);
    back_subtypes.0.insert(back_hero_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: Some(ManaCost::parse("{3}{R}{G}{W}").expect("valid back cost")),
            colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: put a +1/+1 counter on each of up to two target creatures.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            })
            // {3}{R}{G}{W}: Transform Miles Morales. Activate only as a sorcery (face 0).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}{G}{W}: Transform Miles Morales. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}{G}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_miles,
            })
            // Back-face Camouflage — {2}: Put a +1/+1 counter on Ultimate Spider-Man.
            // He gains hexproof and becomes colorless until end of turn (face 1).
            .with_activated_ability(ActivatedAbilityDef {
                text: "Camouflage — {2}: Put a +1/+1 counter on Ultimate Spider-Man. He gains hexproof and becomes colorless until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: camouflage,
            }),
            // GAP: back-face "Whenever you attack, double the number of each kind of
            // counter …" — no double-counters Effect variant; left unwired.
    )
}

fn transform_miles(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn camouflage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Hexproof,
            duration: Duration::EndOfTurn,
        },
        Effect::SetColor {
            target: ctx.source,
            colors: ColorSet::colorless(),
            duration: Duration::EndOfTurn,
        },
    ]
}

fn etb_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets.targets.iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::AddCounters {
                    target: *id,
                    kind: CounterKind::PlusOnePlusOne,
                    count: 1,
                })
            } else {
                None
            }
        })
        .collect()
}
