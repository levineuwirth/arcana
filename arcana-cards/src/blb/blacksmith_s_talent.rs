//! Blacksmith's Talent — `{R}` red Enchantment — Class.
//! Level 1 (ETB): Create a colorless Equipment artifact token named Sword
//!                with "Equipped creature gets +1/+1" and equip {2}.
//! Level 2: At the beginning of combat on your turn, attach target Equipment
//!           you control to up to one target creature you control.
//! Level 3: During your turn, equipped creatures you control have double strike and haste.
//!
//! GAP: Level 1 ETB — custom Equipment artifact token (named Sword with a +1/+1 equip
//! ability and equip cost) is not expressible via Effect::CreateToken (TokenDefinition
//! has no activated-ability slot) or Effect::CreateCommodityToken (Equipment is not a
//! commodity type). Emitting Vec::new() for the ETB.
//! GAP: Level 3 — "equipped creatures you control" subfilter and "during your turn only"
//! duration are not expressible via ContinuousEffect builders. Installing keyword anthems
//! for DoubleStrike and Haste for all creatures as best effort (over-broad).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blacksmith's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let equipment_sub = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    // Level 2 trigger target requirements built here so we can intern "Equipment".
    let equip_target = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::new()
                .with_types(TypeLine::ARTIFACT.into())
                .controlled_by(ControllerConstraint::You)
                .with_subtypes_any(vec![equipment_sub]),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };
    let creature_target = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ),
        count: TargetCount::UpTo(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Level 1 ETB: create Sword Equipment token — GAP
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_sword,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2: begin combat — attach equipment to creature
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: combat_attach,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![equip_target, creature_target],
            })
            // Level 2 activation
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}: Level 2".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up,
            })
            // Level 3 activation
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}: Level 3".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}").unwrap(),
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

fn etb_create_sword(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: custom Equipment token with "+1/+1 for equipped creature" and "equip {2}"
    // is not expressible via TokenDefinition or CreateCommodityToken.
    Vec::new()
}

fn combat_attach(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut it = trig.targets.targets.iter();
    let Some(t_equip) = it.next() else { return Vec::new(); };
    let Some(t_creature) = it.next() else { return Vec::new(); };
    let TargetChoice::Object(equip_id) = t_equip else { return Vec::new(); };
    let TargetChoice::Object(creature_id) = t_creature else { return Vec::new(); };
    vec![Effect::Attach {
        equipment_or_aura: *equip_id,
        target: *creature_id,
    }]
}

fn level_up(
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

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should only apply to "equipped creatures you control" during "your turn" —
    // installing anthems for all creatures you control as over-broad approximation.
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::keyword_anthem(
                ctx.source,
                ctx.controller,
                KeywordAbility::DoubleStrike,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::keyword_anthem(
                ctx.source,
                ctx.controller,
                KeywordAbility::Haste,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
