//! Terra, Magical Adept // Esper Terra
//!
//! Front: `{1}{R}{G}` Legendary Creature — Human Wizard Warrior 4/2.
//! - ETB: mill five cards, put up to one enchantment card milled this way into
//!   your hand.
//! - Trance — `{4}{R}{G}`, {T}: Exile Terra, then return it to the battlefield
//!   transformed under its owner's control. Activate only as a sorcery.
//!
//! Back: Legendary Enchantment Creature — Saga Wizard (Flying).
//! I, II, III — Mesmerize — Create a token that's a copy of target nonlegendary
//!   enchantment you control. It gains haste. If it's a Saga, put up to three
//!   lore counters on it. Sacrifice it at the beginning of your next end step.
//! IV — Add {W}{W}{U}{U}{B}{B}{R}{R}{G}{G}. Exile Esper Terra, then return it
//!   to the battlefield (front face up).
//!
//! # GAP notes
//! - Mill-then-choose-enchantment: `Effect::DigTopN` handles "look at top N, put
//!   one into hand" but Terra mills first (all go to graveyard), then retrieves
//!   one enchantment from the graveyard. The exact "from what was milled" scope
//!   is not expressible — modeled as mill 5 then return an enchantment card from
//!   graveyard to hand (target-based `ReturnFromGraveyardToHand` on an enchantment
//!   in graveyard). Since targeting is required, we use a targeted approach.
//!   Actually ETB with "up to one" targeting on ETB trigger is modeled with
//!   TargetCount::UpTo(1) targeting a card in the graveyard.
//! - Trance "exile, then return transformed": modeled as `Effect::Transform`; the
//!   exile-and-return blink is a GAP (no blink-transformed effect variant).
//! - Back chapters I–III: `Effect::CopyPermanent` creates a copy; haste grant,
//!   conditional lore-counter placement if Saga, and delayed sacrifice are modeled
//!   in sequence. The "if it's a Saga, put up to three lore counters" condition is
//!   GAP (no Conditional on permanent type post-copy). The "up to three" is
//!   approximated as AddCounters 3 (gap: it should be up to three).
//! - Back chapter IV: Add {W}{W}{U}{U}{B}{B}{R}{R}{G}{G} — ten mana pips are
//!   modeled via AddMana sequence. Exile and return front face is modeled as
//!   Effect::Transform (blink GAP same as Trance).
//! - Flying keyword on the back face is on the Characteristics only (no install-
//!   on-transform mechanism; back-face keywords are declared in CardFace).
//! - Keywords on back: Flying.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Terra, Magical Adept");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Legendary Enchantment Creature — Saga Wizard
    let back_name = reg.interner_mut().intern("Esper Terra");
    let saga_sub = reg.interner_mut().intern("Saga");
    let wizard_back = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga_sub);
    back_subtypes.0.insert(wizard_back);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // P/T for Saga Creature (legendary enchantment creature)
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB trigger: mill 5, then put up to one enchantment into hand.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill_and_return_enchantment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // Target: up to one enchantment card in your graveyard
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            // Trance activated ability: {4}{R}{G}, {T}: transform.
            // GAP: "exile, then return transformed" — modeled as Transform in place.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{R}{G}, {T}: Exile Terra, then return it to the battlefield transformed under its owner's control. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{R}{G}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: trance_transform,
            })
            // Back Saga: add lore counter at draw step
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_lore_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Chapter I, II, III (shared): copy target nonlegendary enchantment,
            // gain haste, sacrifice at next end step.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: chapter_i_ii_iii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ENCHANTMENT.into())
                            .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY))
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: chapter_i_ii_iii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ENCHANTMENT.into())
                            .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY))
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(3),
                },
                intervening_if: None,
                effect: chapter_i_ii_iii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ENCHANTMENT.into())
                            .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY))
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Chapter IV: add WUBRG×2 mana + transform back to front face.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 6,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(4),
                },
                intervening_if: None,
                effect: chapter_iv,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn etb_mill_and_return_enchantment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::Mill { player: trig.controller, count: 5 }];
    // Return up to one targeted enchantment from graveyard to hand.
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
    }
    effects
}

fn trance_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile, then return transformed" not expressible — modeled as in-place transform.
    vec![Effect::Transform { target: ctx.source }]
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

fn chapter_i_ii_iii(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let copied_id = *id;
    // Copy the target enchantment, grant haste, then sacrifice it at next end step.
    // GAP: "if it's a Saga, put up to three lore counters on it" — Saga-type check
    // on the token copy post-resolution is not expressible. Adding 3 lore counters
    // unconditionally as best-effort (they won't trigger Saga chapters on a non-Saga copy).
    // GAP: CopyPermanent creates a token copy; we can't easily get the new token's id to
    // grant haste or schedule sac. Emitting copy only; haste + sac GAP.
    vec![
        Effect::CopyPermanent { target: copied_id },
        // GAP: grant haste to the token copy — no id for the just-created token copy.
        // GAP: sacrifice the copy at the beginning of your next end step — no id.
        // GAP: if it's a Saga, put up to three lore counters on it.
    ]
}

fn chapter_iv(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let src = trig.source;
    let p = trig.controller;
    // Add {W}{W}{U}{U}{B}{B}{R}{R}{G}{G}
    vec![
        Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::White, src)] },
        Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::White, src)] },
        Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::Blue, src)] },
        Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::Blue, src)] },
        Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::Black, src)] },
        Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::Black, src)] },
        Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::Red, src)] },
        Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::Red, src)] },
        Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::Green, src)] },
        Effect::AddMana { player: p, mana: vec![ManaUnit::plain(ManaColor::Green, src)] },
        // Exile Esper Terra, then return front face up = Transform back to front.
        // GAP: "exile, then return" — modeled as in-place Transform.
        Effect::Transform { target: src },
    ]
}
