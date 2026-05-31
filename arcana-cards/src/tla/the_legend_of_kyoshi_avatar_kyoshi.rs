//! The Legend of Kyoshi // Avatar Kyoshi — `{4}{G}{G}` Enchantment — Saga (transforming DFC).
//! Front (The Legend of Kyoshi — Enchantment — Saga):
//!   (As this Saga enters and after your draw step, add a lore counter.)
//!   I — Draw cards equal to the greatest power among creatures you control.
//!   II — Earthbend X, where X is the number of cards in your hand. That land becomes
//!        an Island in addition to its other types.
//!   III — Exile this Saga, then return it to the battlefield transformed under your
//!         control.
//! Back (Avatar Kyoshi — Legendary Creature — Avatar):
//!   Lands you control have trample and hexproof.
//!   {T}: Add X mana of any one color, where X is the greatest power among creatures
//!        you control.
//!
//! GAPs:
//! - Chapter II "Earthbend X ... That land becomes an Island": the Earthbend keyword
//!   action (put +1/+1 counters on a land you control / animate it) is not modeled,
//!   and there is no land target to convert. Emitted as a no-op.
//! - Chapter III is modeled as Effect::Transform on the Saga. NOTE: the engine's
//!   automatic final-chapter sacrifice SBA may sacrifice the Saga rather than letting
//!   the transform stand — exile-and-return-transformed is the intended behavior;
//!   only the transform half is expressible here.
//! - Back face static "Lands you control have trample and hexproof" — board-wide
//!   anthem granting keywords to lands; not expressible. Not modeled.
//! - Back face "{T}: Add X mana of any one color" — a mana amount tied to greatest
//!   creature power AND a free color choice is not expressible. Not modeled.
//! - Earthbend keyword: not in the supported keyword surface. Not emitted.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("The Legend of Kyoshi");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Avatar Kyoshi");
    let avatar_sub = reg.interner_mut().intern("Avatar");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(avatar_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // Printed P/T is not specified on the back face; modeled as 0/0 placeholder.
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            // GAP: "Lands you control have trample and hexproof" (board anthem) and
            //   "{T}: Add X mana of any one color" (dynamic mana + color choice) —
            //   not expressible. Not modeled.
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
            // After your draw step, add a lore counter (CR 716.3).
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
            // Chapter I: Draw cards equal to the greatest power among creatures you control.
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
            // Chapter II: Earthbend X (GAP).
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
            // Chapter III: Exile this Saga, then return it transformed under your control.
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

fn chapter_i(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Draw cards equal to the greatest power among creatures you control.
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    let greatest = ids
        .iter()
        .map(|id| script::power_of(state, *id))
        .max()
        .unwrap_or(0)
        .max(0) as u32;
    if greatest == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards {
        player: trig.controller,
        count: greatest,
    }]
}

fn chapter_ii(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Earthbend X, where X is the number of cards in your hand. That land
    // becomes an Island in addition to its other types." — the Earthbend keyword
    // action and the resulting land conversion are not expressible. No-op.
    Vec::new()
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Exile this Saga, then return it to the battlefield transformed under your
    // control." Modeled as a transform of the Saga (the exile-and-return is folded
    // into the in-place transform).
    vec![Effect::Transform { target: trig.source }]
}
