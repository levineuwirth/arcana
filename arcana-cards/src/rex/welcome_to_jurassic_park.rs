//! Welcome to . . . // Jurassic Park (Saga // transforming Legendary Land).
//!
//! Front (Enchantment — Saga, {1}{G}{G}, green):
//!   I  — For each opponent, up to one target noncreature artifact they control becomes a 0/4 Wall
//!        artifact creature with defender for as long as you control this Saga.
//!   II — Create a 3/3 green Dinosaur creature token with trample. It gains haste until end of turn.
//!   III — Destroy all Walls. Exile this Saga, then return it to the battlefield transformed under
//!         your control.
//! Back (Legendary Land — Jurassic Park):
//!   Each Dinosaur card in your graveyard has escape; escape cost = mana cost + exile three other
//!   cards from your graveyard.
//!   {T}: Add {G} for each Dinosaur you control.
//!
//! GAPs:
//! - Chapter I (each opponent picks up to one of their noncreature artifacts and makes it a
//!   0/4 Wall defender for as long as you control this Saga) requires a per-opponent variable
//!   target count plus a control-duration animation that the target spec / Effect catalog do not
//!   express; emitted as an empty effect.
//! - Back face's static "each Dinosaur card in your graveyard has escape" granted-ability and the
//!   "{T}: Add {G} for each Dinosaur you control" mana ability are not expressible (no static
//!   ability-granting effect; mana abilities are not in the activated-ability catalog here). The
//!   land back is declared with its type line only.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;
use arcana_core::objects::NULL_OBJECT_ID;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Welcome to . . .");
    let _ = reg.interner_mut().intern("Dinosaur");
    let _ = reg.interner_mut().intern("Wall");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Jurassic Park — Legendary Land. Its granted-escape static and the
    // tap-for-{G}-per-Dinosaur mana ability are GAPs (see module docs); declare the type line only.
    let back_name = reg.interner_mut().intern("Jurassic Park");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::LAND.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
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

fn add_lore_counter(_: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(_: &GameState, _trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "For each opponent, up to one target noncreature artifact they control becomes a 0/4
    // Wall artifact creature with defender for as long as you control this Saga." Per-opponent
    // variable targeting plus a control-duration type/PT animation are not expressible.
    Vec::new()
}

fn chapter_ii(_: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // Create a 3/3 green Dinosaur token with trample; it gains haste until end of turn.
    let dino = reg.interner().lookup("Dinosaur").expect("Dinosaur interned");
    let mut subs = SubtypeSet::default();
    subs.0.insert(dino);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dino,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: subs,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
            abilities: vec![],
        },
    }]
    // The token is minted with Haste so it can attack the turn it enters (faithful to the
    // "gains haste until end of turn" rider; the token is created during the controller's turn).
}

fn chapter_iii(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // "Destroy all Walls." then exile-and-return-transformed.
    let wall_filter = script::subtype_filter(reg, "Wall");
    let walls = script::ids_matching(state, &wall_filter, trig.controller);
    vec![
        Effect::ForEach {
            targets: walls,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
        // GAP: "exile this Saga, then return it transformed" is approximated by Transform; the
        // engine's final-chapter sacrifice SBA does not model exile-and-return-transformed.
        Effect::Transform { target: trig.source },
    ]
}
