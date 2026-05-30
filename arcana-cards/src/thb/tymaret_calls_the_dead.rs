//! Tymaret Calls the Dead
//!
//! Enchantment — Saga, {2}{B}.
//! I, II — Mill three cards. Then you may exile a creature or enchantment card from your
//! graveyard. If you do, create a 2/2 black Zombie creature token.
//! III — You gain X life and scry X, where X is the number of Zombies you control.
//!
//! GAP: Chapter I/II "you may exile a creature or enchantment card from your graveyard. If you
//! do, create a 2/2 black Zombie creature token" — optional exile-from-graveyard gate not
//! expressible (OptionalPaymentKind has no ExileFromGraveyard variant); modeled as unconditional
//! token creation after mill (fidelity gap — token always created).
//! III: "You gain X life and scry X, where X is the number of Zombies you control" — X is
//! computed dynamically via script::count_matching.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tymaret Calls the Dead");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    // Pre-intern "Zombie" for token creation at resolve time
    let _zombie_sub = reg.interner_mut().intern("Zombie");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
    _state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn make_zombie_token(reg: &CardRegistry, controller: arcana_core::types::PlayerId) -> Effect {
    let zombie = reg.interner().lookup("Zombie").expect("Zombie interned during register()");
    let mut zombie_subtypes = SubtypeSet::default();
    zombie_subtypes.0.insert(zombie);
    Effect::CreateToken {
        controller,
        token: TokenDefinition {
            name: zombie,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: zombie_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }
}

fn chapter_i(
    _state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Mill 3, then (GAP: may exile creature/enchantment from graveyard; if you do) create a
    // 2/2 black Zombie. Modeled as unconditional mill + token.
    vec![
        Effect::Mill {
            player: trig.controller,
            count: 3,
        },
        make_zombie_token(reg, trig.controller),
    ]
}

fn chapter_ii(
    _state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Same as chapter I.
    vec![
        Effect::Mill {
            player: trig.controller,
            count: 3,
        },
        make_zombie_token(reg, trig.controller),
    ]
}

fn chapter_iii(
    state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie_filter = script::subtype_filter(reg, "Zombie")
        .controlled_by(ControllerConstraint::You);
    let x = script::count_matching(state, &zombie_filter, trig.controller);
    vec![
        Effect::GainLife {
            player: trig.controller,
            amount: x,
        },
        Effect::Scry {
            player: trig.controller,
            count: x,
        },
    ]
}
