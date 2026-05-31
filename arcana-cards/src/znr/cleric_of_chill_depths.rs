//! Cleric of Chill Depths — `{1}{U}` 1/3 blue Merfolk Cleric.
//! "Whenever this creature blocks a creature, that creature doesn't
//! untap during its controller's next untap step."
//!
//! "Doesn't untap during its controller's next untap step" is modeled
//! with a stun counter (CR 122.1c / 701.x): a permanent with a stun
//! counter skips its next untap step and removes one such counter
//! instead. "That creature" is the blocked attacker, read via
//! `trig.other_combatant()`.

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
    let name = reg.interner_mut().intern("Cleric of Chill Depths");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
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
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.other_combatant() else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: id,
        kind: CounterKind::Stun,
        count: 1,
    }]
}
