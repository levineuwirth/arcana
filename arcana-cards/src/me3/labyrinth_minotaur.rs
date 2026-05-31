//! Labyrinth Minotaur — `{3}{U}` 1/4 blue Minotaur. "Whenever this
//! creature blocks a creature, that creature doesn't untap during its
//! controller's next untap step."
//!
//! Modeled via `SelfBlocks` + a stun counter on the blocked creature:
//! a creature with a stun counter doesn't untap during its untap step,
//! removing the counter instead — the modern templating for "doesn't
//! untap during its controller's next untap step."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Labyrinth Minotaur");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: stun_blocked_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn stun_blocked_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.other_combatant() else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: id,
        kind: CounterKind::Stun,
        count: 1,
    }]
}
