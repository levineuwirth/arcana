//! Wizard Class — `{U}` blue Enchantment — Class.
//!
//! Level 1: You have no maximum hand size.
//!   GAP: "you have no maximum hand size" is a replacement/static effect not
//!        expressible via install-on-level-up continuous effect builders
//!        (requires a hand-size replacement, not a P/T or keyword anthem).
//!
//! Level 2 ({2}{U}): When this Class becomes level 2, draw two cards.
//!   (Modeled as a triggered ability on CounterAdded to Level 2.)
//!
//! Level 3 ({4}{U}): Whenever you draw a card, put a +1/+1 counter on target creature you control.
//!   GAP: Level 3 "Whenever you draw a card" is a triggered ability that should only fire
//!        while the Class is at level 3; the engine installs all TriggeredAbilityDefs
//!        unconditionally on the CardDefinition. Modeled without level gate as closest
//!        approximation (will fire from level 1 onward).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wizard Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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
            // Level 1 static: "You have no maximum hand size."
            // GAP: static hand-size removal not expressible via continuous-effect builders.
            //
            // Level 2 activation: {2}{U}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").unwrap(),
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
            // Level 3 activation: {4}{U}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{U}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{U}").unwrap(),
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
            })
            // Level 2 "when this Class becomes level 2, draw two cards":
            // modeled as a triggered ability on the Lore counter added to level 2.
            // Note: CounterKind::Level used here; chapter fires when 2nd Level counter placed.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Level),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: on_become_level_2,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 3 "whenever you draw a card, put a +1/+1 counter on target creature you control":
            // GAP: fires from level 1 onward (no level-gate on TriggeredAbilityDef).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_card_drawn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

/// Level 2 activation effect: bump Level counter.
fn level_up_to_2(
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

/// Level 3 activation effect: bump Level counter.
/// "Whenever you draw a card, put a +1/+1 counter on target creature you control"
/// is handled by the triggered ability (on_card_drawn) which is already live.
fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: level-3 "whenever you draw a card" ability is not gated; fires from registration.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}

/// "When this Class becomes level 2, draw two cards."
fn on_become_level_2(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 2,
    }]
}

/// "Whenever you draw a card, put a +1/+1 counter on target creature you control."
fn on_card_drawn(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
