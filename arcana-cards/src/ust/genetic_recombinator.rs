//! Genetic Recombinator — artifact — Contraption (Unstable).
//! "Whenever you crank this Contraption, up to two target creatures
//! each get +2/+2 until end of turn."
//!
//! Contraptions have no mana cost and are cranked from the Contraption
//! deck — the crank trigger has no engine condition. The closest
//! variant (`SelfEntersBattlefield`) is used as a placeholder with a
//! GAP note; the pump effect itself is faithful.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Genetic Recombinator");
    let contraption = reg.interner_mut().intern("Contraption");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "Whenever you crank this Contraption" (Un-set
                // Contraption crank mechanic) has no TriggerCondition variant;
                // SelfEntersBattlefield is the closest placeholder.
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: pump_targets,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            },
        ),
    )
}

fn pump_targets(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::Pump {
                target: *id,
                power: 2,
                toughness: 2,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
            _ => None,
        })
        .collect()
}
