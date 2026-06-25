//! The Legend of Roku // Avatar Roku — transforming Saga.
//!
//! Front (The Legend of Roku, {2}{R}{R} Enchantment — Saga):
//!   I  — Exile the top three cards of your library. Until the end of your next
//!        turn, you may play those cards.
//!   II — Add one mana of any color.
//!   III— Exile this Saga, then return it to the battlefield transformed under
//!        your control.
//! Back (Avatar Roku, Legendary Creature — Avatar):
//!   Firebending 4; {8}: Create a 4/4 red Dragon token with flying and firebending 4.
//!
//! GAP: Chapter I's "exile top three, until end of your next turn you may play those
//! cards" is not an expressible impulse-with-play-window primitive; emitted best-effort
//! as exiling the top three (the play window is the gap). Actually no clean exile-top-N
//! to-exile effect exists either — emitted as Vec::new() for chapter I.
//! GAP: Firebending 4 is not a supported keyword (no KeywordAbility variant).
//! GAP: the back face's {8} create-Dragon activated ability is creature-face engine debt
//! for transformed Sagas; the back face is declared so the catalog records both faces.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::registry::EntersWithSpec;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Legend of Roku");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Avatar Roku, Legendary Creature — Avatar.
    let back_name = reg.interner_mut().intern("Avatar Roku");
    let avatar_sub = reg.interner_mut().intern("Avatar");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(avatar_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // GAP: printed P/T of Avatar Roku unavailable in spec; placeholder.
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            // GAP: Firebending 4 keyword + {8} Dragon activated ability are engine debt.
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

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // I — Exile the top three cards of your library; until end of your next turn you
    // may play those cards.
    // GAP: impulse-exile with a delayed "you may play those cards" window is not an
    // expressible primitive.
    Vec::new()
}

fn chapter_ii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // II — Add one mana of any color.
    // GAP: this is a one-shot Saga-chapter effect, not an activated ability, so
    // the per-color "choose which ability to activate" idiom does not apply, and
    // there is no chosen-color -> AddMana follow-up on Effect::ChooseColor.
    // Emitting red as a best-effort placeholder for the any-color choice.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
    }]
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // III — Exile this Saga, then return it transformed under your control.
    vec![Effect::Transform { target: trig.source }]
}
