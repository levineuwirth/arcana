//! Interface Ace — `{1}{W}` 0/4 Artifact Creature — Robot Pilot.
//! "This creature saddles Mounts and crews Vehicles using its toughness
//! rather than its power." (GAP — saddle/crew-cost static replacement.)
//! "Whenever this creature becomes tapped during your turn, untap it.
//! This ability triggers only once each turn."
//!
//! The becomes-tapped trigger untaps the source, with OncePerTurn
//! frequency. The "during your turn" qualifier has no matching condition
//! helper — GAP'd (fires on any becomes-tapped, once per turn).

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
    let name = reg.interner_mut().intern("Interface Ace");
    let robot = reg.interner_mut().intern("Robot");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "saddles Mounts and crews Vehicles using its toughness rather
    // than its power" — a cost-substitution static, not expressible.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesTapped,
            // GAP (intervening-if): "during your turn" has no matching
            // condition helper.
            intervening_if: None,
            effect: untap_self,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

fn untap_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Untap {
        target: trig.source,
    }]
}
