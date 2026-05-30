//! Cosima, God of the Voyage // The Omenkeel — `{2}{U}` Legendary Creature —
//! God 2/4 (front) // Legendary Artifact — Vehicle (back).
//!
//! Front face (Cosima, God of the Voyage):
//! At the beginning of your upkeep, you may exile Cosima. If you do, it gains
//! "Whenever a land you control enters, if Cosima is exiled, you may put a
//! voyage counter on it. If you don't, return Cosima to the battlefield with
//! X +1/+1 counters on it and draw X cards, where X is the number of voyage
//! counters on it."
//!
//! Back face (The Omenkeel): Legendary Artifact — Vehicle.
//! Whenever a Vehicle you control deals combat damage to a player, that player
//! exiles that many cards from the top of their library. You may play lands
//! from among those cards for as long as they remain exiled.
//! Crew 1.
//!
//! # GAPs
//! - Front face upkeep ability: "you may exile Cosima; if you do, it gains a
//!   triggered ability while in exile" — self-exile with a triggered ability
//!   active in exile zone is not modelable. Emitting Vec::new().
//! - Back face "whenever a Vehicle you control deals combat damage to a player,
//!   that player exiles that many cards..." — this is a back-face-only
//!   triggered ability and the "exile N, may play lands from exile" rider is
//!   not in the effect catalog. GAP: back-face-only triggered ability not
//!   modeled.
//! - Crew keyword not in engine keyword list (GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cosima, God of the Voyage");
    let god_sub = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // Back face: The Omenkeel — Legendary Artifact — Vehicle
    let back_name = reg.interner_mut().intern("The Omenkeel");
    let vehicle_sub = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle_sub);
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::blue(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: back_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    let back = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_cosima,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face-only triggered ability (Vehicle combat damage →
        //      exile top N cards, may play lands) not modeled
        // GAP: Crew keyword not in engine keyword list
    )
}

fn upkeep_cosima(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may exile Cosima; if you do, it gains a triggered ability
    // while in exile" — self-exile with triggered ability active in exile
    // zone is not modelable with the current engine API.
    Vec::new()
}
