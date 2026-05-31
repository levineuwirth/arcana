//! The Kami War // O-Kagachi Made Manifest — {1}{W}{U}{B}{R}{G} Enchantment — Saga (WUBRG).
//!
//! I  — Exile target nonland permanent an opponent controls.
//! II — Return up to one other target nonland permanent to its owner's hand.
//!      Then each opponent discards a card.
//! III — Exile this Saga, then return it to the battlefield transformed under
//!       your control.
//! Back (O-Kagachi Made Manifest, Enchantment Creature — Dragon Spirit, all colors,
//!       flying, trample): Whenever this creature attacks, defending player chooses
//!       a nonland card in your graveyard. Return that card to your hand. This
//!       creature gets +X/+0 until end of turn, where X is that card's mana value.
//!
//! GAP: Chapter III's "exile, then return transformed under your control" is not
//! auto-wired (CR 716 saga-transform is engine debt). Emitting Effect::Transform on
//! trig.source as a best effort; the final-chapter sacrifice SBA may still fire.
//! GAP: back-face attack trigger ("defending player chooses a nonland card in your
//! graveyard, return it, +X/+0 where X is its mana value") has no expressible trigger
//! condition (no defending-player-chooses-from-graveyard primitive) — not wired.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Kami War");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let all_colors =
        ColorSet::white() | ColorSet::blue() | ColorSet::black() | ColorSet::red() | ColorSet::green();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: all_colors,
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: O-Kagachi Made Manifest — Enchantment Creature — Dragon Spirit.
    let back_name = reg.interner_mut().intern("O-Kagachi Made Manifest");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(dragon_sub);
    back_subtypes.0.insert(spirit_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: all_colors,
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
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
            // Add a lore counter at the beginning of your first main phase.
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
            // I — Exile target nonland permanent an opponent controls.
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
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // II — Return up to one other target nonland permanent to its owner's
            //      hand. Then each opponent discards a card.
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
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            // III — Exile this Saga, then return it transformed under your control.
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

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}

fn chapter_ii(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effs = Vec::new();
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effs.push(Effect::ReturnToHand { target: *id });
    }
    // Then each opponent discards a card.
    for p in script::opponents(state, trig.controller) {
        effs.push(Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    effs
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile, then return transformed under your control" is not auto-wired;
    // best-effort flip to the back (creature) face in place.
    vec![Effect::Transform { target: trig.source }]
}
