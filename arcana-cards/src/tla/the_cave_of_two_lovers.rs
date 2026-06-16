//! The Cave of Two Lovers — `{3}{R}` Enchantment — Saga (red)
//!
//! I — Create two 1/1 white Ally creature tokens.
//! II — Search your library for a Mountain or Cave card, reveal it, put it into
//!      your hand, then shuffle.
//! III — Earthbend 3. (Target land you control becomes a 0/0 creature with haste
//!       that's still a land. Put three +1/+1 counters on it. When it dies or is
//!       exiled, return it to the battlefield tapped.)
//!
//! GAP: Earthbend keyword (Chapter III) is not implemented in the engine.
//! GAP: Scryfall keyword "Earthbend" not in engine keyword surface.
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Cave of Two Lovers");
    let saga_sub = reg.interner_mut().intern("Saga");
    // Pre-intern strings needed at resolve time.
    let _ = reg.interner_mut().intern("Ally");
    let _ = reg.interner_mut().intern("Mountain");
    let _ = reg.interner_mut().intern("Cave");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Create two 1/1 white Ally creature tokens.
    let ally_id = reg.interner().lookup("Ally").expect("Ally interned at register");
    let mut token_sub = SubtypeSet::default();
    token_sub.0.insert(ally_id);
    let token = TokenDefinition {
        name: ally_id,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_sub,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}

fn chapter_ii(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Search for a Mountain or Cave land card.
    let mountain = reg.interner().lookup("Mountain");
    let cave = reg.interner().lookup("Cave");
    let st_ids: Vec<_> = [mountain, cave].into_iter().flatten().collect();

    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_subtypes_any(st_ids);

    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}

fn chapter_iii(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Earthbend 3 — keyword not implemented in the engine.
    Vec::new()
}
