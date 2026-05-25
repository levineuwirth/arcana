//! Vengeful Tracker — `{1}{R}` 2/2 red creature. "Whenever an opponent
//! sacrifices an artifact, this creature deals 2 damage to them."
//!
//! GAP: trigger — Sacrificed does not have a "who" field to restrict to
//! opponents. Triggering caster used in effect as best-effort.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vengeful Tracker");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: Sacrificed has no "who" field; fires for any player sacrificing
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                },
                intervening_if: None,
                effect: opponent_sac_artifact_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn opponent_sac_artifact_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponent = trig.triggering_caster().unwrap_or(trig.controller);
    vec![Effect::DealDamage {
        target: DamageTarget::Player(opponent),
        amount: 2,
        source: trig.source,
    }]
}
