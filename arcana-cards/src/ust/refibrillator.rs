//! Refibrillator — Artifact — Contraption (Unstable, 2017).
//! "Whenever you crank this Contraption, return target creature card from
//! your graveyard to your hand." Contraption cranking (assembly/sprockets)
//! is not modeled — there is no crank trigger condition. Per the trigger
//! catalog's closest-variant rule the trigger is wired on
//! `SelfBecomesTapped` with a GAP note; a Contraption with no tap ability
//! is effectively inert, so this never over-fires in practice.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Refibrillator");
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
                // GAP: trigger — "Whenever you crank this Contraption"
                // (Un-set Contraption cranking is not modeled; closest
                // variant used as a placeholder).
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: raise_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            },
        ),
    )
}

fn raise_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
