//! The Legend of Kuruk // Avatar Kuruk — {2}{U}{U} Enchantment — Saga (transforming DFC).
//!
//! Front (Saga):
//!   I, II — Scry 2, then draw a card.
//!   III — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back (Avatar Kuruk, Legendary Creature — Avatar):
//!   Whenever you cast a spell, create a 1/1 colorless Spirit creature token with
//!     "This token can't block or be blocked by non-Spirit creatures."
//!   Exhaust — Waterbend {20}: Take an extra turn after this one.
//!
//! GAPs:
//! - Chapter III "Exile, then return transformed under your control" is approximated with
//!   Effect::Transform on the Saga's source. The engine has no exile-and-return-transformed
//!   primitive, and the automatic final-chapter sacrifice SBA (CR 716.5d) is NOT suppressed for
//!   transforming Sagas — so the flip-instead-of-sacrifice semantics are engine debt.
//! - Spirit token's "can't block or be blocked by non-Spirit creatures" rider is not expressible
//!   on a TokenDefinition (no per-token static ability surface) — minted as a bare 1/1 Spirit.
//! - "Exhaust — Waterbend {20}" activation cost (once-per-game exhaust + waterbend alternate cost)
//!   is not expressible as an activated-ability cost; the extra-turn body is omitted as an
//!   authored ability. GAP: exhaust + waterbend cost mechanics unmodeled.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Legend of Kuruk");
    let saga_sub = reg.interner_mut().intern("Saga");
    let avatar = reg.interner_mut().intern("Avatar");
    let _spirit = reg.interner_mut().intern("Spirit");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Avatar Kuruk, Legendary Creature — Avatar (printed 5/5).
    let back_name = reg.interner_mut().intern("Avatar Kuruk");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(avatar);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(arcana_core::registry::EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            // First main phase: add a lore counter.
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
            // Chapter I — Scry 2, then draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: chapter_scry_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Chapter II — Scry 2, then draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: chapter_scry_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Chapter III — exile, then return transformed (approximated via Transform).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(3),
                },
                intervening_if: None,
                effect: chapter_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back-face: whenever you cast a spell, create a 1/1 Spirit token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_spirit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 5 (cast-a-spell Spirit maker) is back-face only.
            .with_trigger_face_gate(5, 1),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_scry_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::Scry {
            player: trig.controller,
            count: 2,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ]
}

fn chapter_transform(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: should exile and return transformed under your control; approximated with a flip.
    vec![Effect::Transform {
        target: trig.source,
    }]
}

fn make_spirit(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned at register");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    // GAP: token's "can't block or be blocked by non-Spirit creatures" rider not expressible.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spirit,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
