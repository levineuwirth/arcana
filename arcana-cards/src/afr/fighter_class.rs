//! Fighter Class — `{R}{W}` red/white Enchantment — Class.
//!
//! Level 1 (base):
//!   When this Class enters, search your library for an Equipment card,
//!   reveal it, put it into your hand, then shuffle.
//!
//! Level 2 ({1}{R}{W}):
//!   Equip abilities you activate cost {2} less to activate.
//!
//! Level 3 ({3}{R}{W}):
//!   Whenever a creature you control attacks, up to one target creature
//!   blocks it this combat if able.
//!
//! # GAPs
//! - Level 2 "Equip abilities cost {2} less" is a cost-reduction continuous
//!   effect; not expressible as a P/T or keyword anthem.
//!   // GAP: Level 2 equip cost reduction — continuous-effect engine debt
//!   (cost reduction not in ContinuousEffect builders).
//! - Level 3 "target creature blocks [attacker] if able" is a forced-block
//!   static/trigger effect not in the engine catalog.
//!   // GAP: Level 3 forced-block trigger — not expressible with current
//!   Effect catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fighter Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 1 ETB: search for Equipment card
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_equipment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Level 2: {1}{R}{W}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}{W}: Level 2. Equip abilities you activate cost {2} less.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}{W}").unwrap(),
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
            // Level 3: {3}{R}{W}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}{W}: Level 3. Whenever a creature you control attacks, up to one target creature blocks it this combat if able.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}{W}").unwrap(),
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

fn etb_tutor_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Search library for an Equipment card (type-line: Artifact + Equipment subtype)
    // ObjectFilter does not have a dedicated "Equipment" subtype filter; use
    // the canonical artifact type filter as the closest expressible approximation.
    // GAP: Equipment subtype filter not directly expressible; using artifact filter.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into()),
        reveal: true,
    }]
}

fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 2 equip cost reduction — continuous-effect engine debt
    // (cost reduction not in ContinuousEffect builders).
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
    // GAP: Level 3 forced-block trigger — not expressible with current
    // Effect catalog.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}
