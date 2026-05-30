//! Bard Class — `{R}{G}` red/green Enchantment — Class.
//!
//! Level 1 (base): Legendary creatures you control enter with an additional
//! +1/+1 counter on them.
//! GAP: Level 1 ETB replacement "enter with an additional +1/+1 counter" is
//! not expressible (replacement effect engine subsystem deferred).
//!
//! Level 2 ({R}{G}): Legendary spells you cast cost {R}{G} less to cast.
//! GAP: Level 2 cost-reduction static not expressible (continuous-effect /
//! cost-reduction subsystem deferred).
//!
//! Level 3 ({3}{R}{G}): Whenever you cast a legendary spell, exile the top
//! two cards of your library. You may play them this turn.
//! GAP: "exile and may play this turn" (impulse-exile) is not in the Effect
//! catalog; the trigger is wired but its body returns Vec::new().

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bard Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
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
            // Level 2: {R}{G} (requires level 1)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{G}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{G}").unwrap(),
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
            // Level 3: {3}{R}{G} (requires level 2)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}{G}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}{G}").unwrap(),
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
            // Level 3 triggered ability: whenever you cast a legendary spell
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: legendary_spell_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 2 cost-reduction "Legendary spells cost {R}{G} less" is
    // not expressible (continuous cost-reduction subsystem deferred).
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

fn legendary_spell_cast(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top two cards of your library; you may play them this
    // turn" (impulse-exile) is not in the Effect catalog.
    Vec::new()
}
