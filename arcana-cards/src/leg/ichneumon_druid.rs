//! Ichneumon Druid — `{1}{G}{G}` 1/1 green Human Druid.
//! "Whenever an opponent casts an instant spell other than the first instant
//! spell that player casts each turn, this creature deals 4 damage to that player."
//! GAP: "other than the first spell per turn" per-player count tracking not in engine.

use arcana_core::effects::Effect;
use arcana_core::events::GameEvent;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ichneumon Druid");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types_any: Some(TypeLine(TypeLine::INSTANT)),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: on_opponent_instant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_opponent_instant(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "other than the first instant spell that player casts each turn"
    // per-player per-turn spell-count tracking not in engine. Best-effort: always trigger.
    let Some(caster) = trig.triggering_caster() else { return Vec::new(); };
    use arcana_core::events::DamageTarget;
    vec![Effect::DealDamage {
        target: DamageTarget::Player(caster),
        amount: 4,
        source: trig.source,
    }]
}
