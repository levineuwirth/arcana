//! Horn of the Mark — `{2}` Legendary Artifact (LTR).
//! "Whenever two or more creatures you control attack a player, look
//! at the top five cards of your library. You may reveal a creature
//! card from among them and put it into your hand. Put the rest on
//! the bottom of your library in a random order."
//!
//! GAP: the "two or more creatures ... attack a player" count
//! condition is not expressible — the closest trigger is
//! `CreatureAttacks` (fires per attacking creature); modeled with
//! `TriggerFrequency::OncePerTurn` to avoid one fire per attacker,
//! but the two-attacker minimum is unmodeled.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Horn of the Mark");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: "Whenever two or more creatures you control attack
                // a player" — no attacker-count trigger; CreatureAttacks
                // fires per attacker, capped to once per turn.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: dig_for_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn dig_for_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 5,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}
