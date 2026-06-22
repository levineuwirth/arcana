//! Trap Digger — `{3}{W}` 1/3 Human Soldier.
//! "{2}{W}, {T}: Put a trap counter on target land you control.
//!  Sacrifice a land with a trap counter on it: This creature deals 3 damage
//!  to target attacking creature without flying."
//!
//! Two activated abilities. The first is a mana+tap activation that puts a
//! named "trap" counter on a target land you control. The second is a
//! "sacrifice a land [with a trap counter]" activation that pings a target
//! attacking creature for 3 — the cost's "with a trap counter on it" rider is
//! a GAP (ActivationCost.sacrifice_other can filter by type, not by counter),
//! and "without flying" is approximated by a no-flying filter via without_keyword.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trap Digger");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let _trap = reg.interner_mut().intern("trap");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}, {T}: Put a trap counter on target land you control."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_trap_counter,
            })
            // GAP: the activation cost "Sacrifice a land WITH A TRAP COUNTER on it"
            // can't filter by counter presence (sacrifice_other filters by type
            // only); modeled here as "Sacrifice a land". "without flying" is
            // expressed via without_keyword on the target filter.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a land with a trap counter on it: This creature \
                       deals 3 damage to target attacking creature without flying."
                    .into(),
                cost: ActivationCost {
                    sacrifice_other: Some(
                        ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .attacking_only()
                            .without_keyword(arcana_core::effects::KeywordAbility::Flying),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_three,
            }),
    )
}

fn put_trap_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let Some(trap) = reg.interner().lookup("trap") else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(trap),
        count: 1,
    }]
}

fn deal_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 3,
    }]
}
