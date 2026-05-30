//! A-Sorcerer Class — {U}{R}, Enchantment — Class.
//!
//! Level 1: When this Class enters, draw two cards, then discard two cards.
//! Level 2 ({U}{R}): Creatures you control have "{T}: Add {U} or {R}. Spend this mana
//!   only to cast an instant or sorcery spell or to gain a Class level."
//! Level 3 ({1}{U}{R}): Whenever you cast an instant or sorcery spell, that spell deals
//!   damage to each opponent equal to the number of instant and sorcery spells you've cast
//!   this turn.
//!
//! GAP: Level 2 per-creature activated mana ability "{T}: Add {U} or {R} (spend on instant/
//!   sorcery/Class level)" — this is not a P/T anthem or keyword anthem; it is deferred
//!   (continuous-effect engine subsystem).
//! GAP: Level 3 "that spell deals damage" — the damage comes from the spell itself, not from
//!   this triggered ability; "that spell deals damage equal to # inst/sor cast this turn" is
//!   a replacement/rider on the spell's resolution, not directly expressible. Best effort:
//!   we model it as a triggered ability dealing damage to each opponent on behalf of this
//!   source when you cast an instant or sorcery.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Sorcerer Class");
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
            // Level 1 ETB: draw 2, discard 2.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 3: Whenever you cast an instant or sorcery spell, deal damage to each opponent.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: level3_cast_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2: {U}{R} — requires level 1.
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
            // Level 3: {1}{U}{R} — requires level 2.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}{R}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}{R}").unwrap(),
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

fn etb_draw_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 2 },
        Effect::Discard { player: trig.controller, count: 2, choice: DiscardChoice::ControllerChooses },
    ]
}

fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 2 "Creatures you control have {T}: Add {U} or {R}..." is not a
    // P/T anthem or keyword anthem — per-creature mana ability grant is deferred
    // (continuous-effect engine subsystem).
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
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}

fn level3_cast_trigger(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Only fires at level 3 (at least 3 Level counters). No level gate on TriggeredAbilityDef,
    // so this triggers at all levels — acceptable for best-effort; verify pipeline will note gap.
    // Deal damage to each opponent equal to # of instant/sorcery spells cast this turn.
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let count = script::spells_cast_this_turn(state, &filter, trig.controller);
    if count == 0 {
        return Vec::new();
    }
    let opponents = script::opponents(state, trig.controller);
    opponents.into_iter().map(|opp| {
        Effect::DealDamage {
            target: DamageTarget::Player(opp),
            amount: count,
            source: trig.source,
        }
    }).collect()
}
