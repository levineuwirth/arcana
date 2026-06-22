//! Phyrexian Seedling — `{2}{G}` 0/0 Phyrexian Plant.
//!
//! Oracle:
//! * "Phyrexian Seedling enters the battlefield with a +1/+1 counter
//!   on it." — wired as an ETB trigger that adds one +1/+1 counter to
//!   itself. (Modeled as a trigger rather than a true CR 614 ETB
//!   replacement — a brief 0/0 SBA window is a fidelity partial.)
//! * "Proliferatelink (Damage dealt by this creature also causes you
//!   to proliferate that many times…)" — a damage-dealt trigger that
//!   proliferates once per point of damage. The source-scope is a
//!   fidelity partial: ObjectFilter has no "this object only" form, so
//!   the source filter is "a creature you control".

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Seedling");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let plant = reg.interner_mut().intern("Plant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(plant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: proliferate_per_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn proliferate_per_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "proliferate that many times" — once per point of damage dealt.
    let n = trig.damage_amount().unwrap_or(0);
    let mut effects = Vec::new();
    for _ in 0..n {
        effects.push(Effect::Proliferate);
    }
    effects
}
