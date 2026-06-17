//! Bulwark Ox — `{1}{W}` 2/2 Ox Mount.
//!
//! * Whenever this creature attacks while saddled, put a +1/+1 counter on target
//!   creature. (The "while saddled" gate is GAP'd — no saddled-state trigger
//!   condition; fires on every attack.)
//! * Sacrifice this creature: Creatures you control with counters on them gain
//!   hexproof and indestructible until end of turn. (The "with counters" filter is
//!   GAP'd — no counter predicate in scope; applies to all creatures you control.)
//! * Saddle 1 — not a modeled keyword; GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bulwark Ox");
    let ox = reg.interner_mut().intern("Ox");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ox);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "while saddled" condition not expressible; fires on every attack.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice this creature: Creatures you control with counters on them gain hexproof and indestructible until end of turn.".into(),
                cost: ActivationCost { sacrifice: true, ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: protect_counter_creatures,
            }),
    )
}

fn attack_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 1 }]
}

fn protect_counter_creatures(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "with counters on them" filter not expressible; applies to all your creatures.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![
        Effect::ForEach {
            targets: ids.clone(),
            effect: Box::new(Effect::GrantKeyword {
                target: NULL_OBJECT_ID,
                keyword: KeywordAbility::Hexproof,
                duration: Duration::EndOfTurn,
            }),
        },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::GrantKeyword {
                target: NULL_OBJECT_ID,
                keyword: KeywordAbility::Indestructible,
                duration: Duration::EndOfTurn,
            }),
        },
    ]
}
