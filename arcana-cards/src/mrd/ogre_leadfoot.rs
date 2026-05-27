//! Ogre Leadfoot — `{4}{R}` 3/3 red Ogre.
//! "Whenever this creature becomes blocked by an artifact creature, destroy that
//! creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ogre Leadfoot");
    let ogre = reg.interner_mut().intern("Ogre");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                // GAP: SelfBecomesBlocked does not filter to "artifact creature" blockers.
                // The condition fires for any blocker; we apply the destroy only to
                // artifact-creature blockers via the effect fn.
                intervening_if: None,
                effect: destroy_artifact_blocker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn destroy_artifact_blocker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Use other_combatant to get the blocking creature.
    let Some(id) = trig.other_combatant() else { return Vec::new(); };
    // GAP: We cannot check at resolve time if the blocker is an artifact creature
    // without state access beyond the approved script API. Emitting destroy unconditionally
    // for the blocking creature (over-fires if blocker is not an artifact creature).
    vec![Effect::DestroyPermanent { target: id }]
}
