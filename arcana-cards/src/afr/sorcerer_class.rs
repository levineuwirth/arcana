//! Sorcerer Class — `{U}{R}` Enchantment — Class.
//! Level 1 (ETB): When this Class enters, draw two cards, then discard two cards.
//! Level 2 ({U}{R}): Creatures you control have "{T}: Add {U} or {R}. Spend this
//! mana only to cast an instant or sorcery spell or to gain a Class level."
//! Level 3 ({3}{U}{R}): Whenever you cast an instant or sorcery spell, that spell
//! deals damage to each opponent equal to the number of instant and sorcery spells
//! you've cast this turn.
//!
//! GAP: Level 2 grants a creature-mounted mana ability ("creatures you control have
//! {T}: Add {U} or {R}...") — per-creature granted activated mana ability is not
//! expressible via InstallContinuousEffect anthems; omitted.
//! GAP: Level 3 grants a "whenever you cast an instant or sorcery, deal damage equal
//! to spells cast this turn" effect — this requires a triggered ability gated on
//! the Class being at level 3, which is not expressible with the current install-on-
//! level-up continuous-effect model. Omitted.

use arcana_core::effects::{Effect, DiscardChoice};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorcerer Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // ETB trigger: draw two cards, then discard two cards (level 1 static)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_enter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2: {U}{R} — requires level 1 (min_self_counters = 1)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{R}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{R}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_2,
            })
            // Level 3: {3}{U}{R} — requires level 2 (min_self_counters = 2)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}{R}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}{R}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            }),
    )
}

fn on_enter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 2 },
        Effect::Discard {
            player: trig.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 2 grants creatures a mana tap ability ("Add {U} or {R}...") —
    // per-creature granted activated mana ability not expressible via current
    // ContinuousEffect catalog. Only the level counter is added.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 3 grants "whenever you cast an instant or sorcery, that spell
    // deals damage to each opponent equal to the number of instant/sorcery spells
    // cast this turn" — triggered ability gated on level ≥ 3 is not expressible.
    // Only the level counter is added.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}
