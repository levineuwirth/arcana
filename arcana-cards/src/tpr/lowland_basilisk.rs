//! Lowland Basilisk — `{2}{G}` 1/3 green Basilisk.
//! "Whenever this creature deals damage to a creature, destroy that creature at end
//! of combat."
//! GAP: "destroy at end of combat" delayed trigger — DelayedWhen has no EndOfCombat
//! variant; using DestroyPermanent immediately as closest approximation.

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
    let name = reg.interner_mut().intern("Lowland Basilisk");
    let basilisk = reg.interner_mut().intern("Basilisk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(basilisk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .with_types(TypeLine::CREATURE.into())
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Creature,
                    combat_only: false,
                },
                intervening_if: None,
                effect: destroy_damaged_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn destroy_damaged_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "at end of combat" timing not modeled; DestroyPermanent fires immediately.
    let Some(id) = trig.other_combatant() else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: id }]
}
