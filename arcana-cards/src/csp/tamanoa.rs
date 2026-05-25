//! Tamanoa — `{R}{G}{W}` 2/4 red/green/white Spirit. "Whenever a noncreature
//! source you control deals damage, you gain that much life."
//!
//! GAP: no TriggerCondition for "noncreature source deals damage". The
//! DamageDealt condition's source_filter cannot easily be "noncreature you
//! control". Best-effort using DamageDealt with source filter; the amount
//! accessor trig.damage_amount() is used.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tamanoa");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You)
                        .without_types(TypeLine::CREATURE.into()),
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: on_noncreature_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_noncreature_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    vec![Effect::GainLife { player: trig.controller, amount: n }]
}
