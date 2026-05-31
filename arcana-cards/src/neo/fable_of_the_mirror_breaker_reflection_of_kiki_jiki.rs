//! Fable of the Mirror-Breaker // Reflection of Kiki-Jiki
//! (transforming Saga, layout "transform")
//!
//! Front face: Fable of the Mirror-Breaker — {2}{R} Enchantment — Saga.
//!   I — Create a 2/2 red Goblin Shaman creature token with "Whenever this token
//!       attacks, create a Treasure token."
//!   II — You may discard up to two cards. If you do, draw that many cards.
//!   III — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back face: Reflection of Kiki-Jiki — Enchantment Creature — Goblin Shaman, 2/2.
//!   {1}, {T}: Create a token that's a copy of another target nonlegendary creature
//!       you control, except it has haste. Sacrifice it at the beginning of the next end step.
//!
//! GAP (chapter I): the token's printed triggered ability ("Whenever this token attacks,
//!   create a Treasure token") cannot be attached to a `TokenDefinition` via the
//!   demonstrated API (no ability-bearing token-attack trigger). The 2/2 red Goblin
//!   Shaman body is emitted; the attack trigger is engine debt.
//! GAP (chapter II): "discard UP TO two, then draw that many" couples a variable discard
//!   to a variable draw; the demonstrated discard/draw effects take fixed counts and the
//!   draw cannot read the number actually discarded. Emitted as a no-op.
//! GAP (chapter III): "Exile this Saga, then return it transformed" — modeled with the
//!   closest primitive `Effect::Transform`. The exact exile-and-return-transformed flow
//!   (and its interaction with the automatic final-chapter sacrifice SBA) is engine debt.
//! GAP (back activated ability): the copy's "except it has haste" + "sacrifice at the next
//!   end step" riders cannot be attached to the new token id produced by `CopyPermanent`.
//!   `Effect::CopyPermanent` is emitted; haste and the delayed sacrifice are engine debt.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fable of the Mirror-Breaker");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    // Pre-intern token / back-face subtypes for resolution.
    let goblin = reg.interner_mut().intern("Goblin");
    let shaman = reg.interner_mut().intern("Shaman");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Reflection of Kiki-Jiki — Enchantment Creature — Goblin Shaman, 2/2.
    let back_name = reg.interner_mut().intern("Reflection of Kiki-Jiki");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(goblin);
    back_subtypes.0.insert(shaman);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            // After your draw step, add a lore counter. (CR 716.3)
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
            // Chapter I: create a 2/2 red Goblin Shaman token.
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
            // Chapter II: discard up to two, draw that many.
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
            // Chapter III: exile this Saga, then return it transformed.
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
            })
            // Back-face activated ability: {1}, {T}: copy another nonlegendary creature you control.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Create a token that's a copy of another target nonlegendary creature you control, except it has haste. Sacrifice it at the beginning of the next end step.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You)
                            .without_supertypes(
                                SupertypeSet::new().with(SupertypeSet::LEGENDARY),
                            ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: kiki_copy,
            }),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let goblin = reg.interner().lookup("Goblin").expect("interned at register");
    let shaman = reg.interner().lookup("Shaman").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(goblin);
    token_subtypes.0.insert(shaman);
    // GAP: token's "Whenever this token attacks, create a Treasure token" trigger is
    // not attachable to a TokenDefinition via the demonstrated API.
    let token = TokenDefinition {
        name: goblin,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

fn chapter_ii(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "You may discard up to two cards. If you do, draw that many cards." — a
    // variable discard coupled to an equal-count draw is not expressible with the
    // fixed-count discard/draw effects in the demonstrated API.
    Vec::new()
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Exile this Saga, then return it to the battlefield transformed under your
    // control." Modeled with the closest primitive: flip to the back face.
    vec![Effect::Transform {
        target: trig.source,
    }]
}

fn kiki_copy(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "except it has haste" and "sacrifice it at the next end step" riders cannot
    // be attached to the token id produced by CopyPermanent.
    vec![Effect::CopyPermanent { target: *id }]
}
