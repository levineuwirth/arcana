//! Elesh Norn // The Argent Etchings — `{2}{W}{W}` Legendary Creature — Phyrexian Praetor 3/5.
//! Vigilance.
//! Whenever a source an opponent controls deals damage to you or a permanent you control,
//!   that source's controller loses 2 life unless they pay {1}.
//! {2}{W}, Sacrifice three other creatures: Exile Elesh Norn, then return it to the battlefield
//!   transformed under its owner's control. Activate only as a sorcery.
//! Back face: The Argent Etchings — Enchantment — Saga.
//!   I — Incubate 2 five times, then transform all Incubator tokens you control.
//!   II — Creatures you control get +1/+1 and gain double strike until end of turn.
//!   III — Destroy all other permanents except artifacts, lands, and Phyrexians. Exile this Saga,
//!         then return it to the battlefield (front face up).
//!
//! # GAPs
//! - "Whenever a source an opponent controls deals damage..." — DamageDealt trigger
//!   with opponent-source constraint is not modeled. GAP: trigger omitted.
//! - Activated ability "{2}{W}, Sacrifice three other creatures:" — sacrifice-N as
//!   an activation cost is not expressible (OptionalPaymentKind only has Mana/Life).
//!   GAP: activation omitted.
//! - Back face Chapter I: "Incubate 2 five times, then transform all Incubator tokens" —
//!   transform-all is not expressible (Transform takes a single target id). Incubate x5 modeled;
//!   transform-all is a GAP.
//! - Back face Chapter III: "Destroy all other permanents except artifacts, lands, Phyrexians"
//!   — wired: non-artifact, non-land, and `.without_subtype_sym(Phyrexian)`.
//! - Back face Chapter III: "Exile this Saga, then return it front face up" — modeled as
//!   Transform (back to front).
//! - Back face's triggered abilities (Chapters) are authored via the CardDefinition and will be
//!   active regardless of face. Engine doesn't support face-gated triggered abilities.
//! - Incubate keyword listed by Scryfall — not a keyword ability variant; effect used directly.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elesh Norn");
    let praetor_sub = reg.interner_mut().intern("Praetor");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian_sub);
    subtypes.0.insert(praetor_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // Back face: The Argent Etchings — Enchantment Saga
    let back_name = reg.interner_mut().intern("The Argent Etchings");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // GAP: "Whenever a source an opponent controls deals damage to you or a permanent
            //   you control" — DamageDealt trigger with opponent-source constraint not modeled.
            // GAP: Activated ability "{2}{W}, Sacrifice three other creatures:" — sacrifice-N
            //   activation cost not expressible; activation omitted.

            // Back face: Saga triggers (active on back face only in rules, but modeled globally)
            // Add lore counter trigger for the Saga back face
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_lore_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: chapter_i,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: chapter_ii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(3),
                },
                intervening_if: None,
                effect: chapter_iii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_lore_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Incubate 2 five times, then transform all Incubator tokens.
    // GAP: transform all Incubator tokens — Transform requires a single target id;
    //   "transform all" is not expressible. Only the Incubate x5 is modeled.
    vec![
        Effect::Incubate { controller: trig.controller, n: 2 },
        Effect::Incubate { controller: trig.controller, n: 2 },
        Effect::Incubate { controller: trig.controller, n: 2 },
        Effect::Incubate { controller: trig.controller, n: 2 },
        Effect::Incubate { controller: trig.controller, n: 2 },
    ]
}

fn chapter_ii(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Creatures you control get +1/+1 and gain double strike until end of turn.
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .flat_map(|id| {
            vec![
                Effect::Pump {
                    target: id,
                    power: 1,
                    toughness: 1,
                    duration: Duration::EndOfTurn,
                    keywords: vec![],
                },
                Effect::GrantKeyword {
                    target: id,
                    keyword: KeywordAbility::DoubleStrike,
                    duration: Duration::EndOfTurn,
                },
            ]
        })
        .collect()
}

fn chapter_iii(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Destroy all other permanents except artifacts, lands, and Phyrexians.
    // "except Phyrexians" wired via without_subtype_sym.
    let mut filter = ObjectFilter::new()
        .without_types(TypeLine(TypeLine::ARTIFACT | TypeLine::LAND));
    if let Some(phyrexian) = reg.interner().lookup("Phyrexian") {
        filter = filter.without_subtype_sym(phyrexian);
    }
    let ids = script::ids_matching(state, &filter, trig.controller);
    // Exclude self
    let mut effects: Vec<Effect> = ids
        .into_iter()
        .filter(|&id| id != trig.source)
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect();
    // Exile this Saga, then return it front face up — modeled as Transform (back→front)
    effects.push(Effect::Transform { target: trig.source });
    effects
}
