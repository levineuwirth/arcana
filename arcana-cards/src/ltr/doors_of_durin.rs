//! Doors of Durin — {3}{R}{G} Legendary Artifact (The Lord of the
//! Rings: Tales of Middle-earth, 2023). "Whenever you attack, scry 2,
//! then you may reveal the top card of your library. If it's a
//! creature card, put it onto the battlefield tapped and attacking.
//! Until your next turn, it gains trample if you control a Dwarf and
//! hexproof if you control an Elf." Approximated as an attack trigger
//! that scrys 2 and reveals/deploys a creature from the top.

use arcana_core::effects::{DigRest, Effect, RevealDest};
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
    let name = reg.interner_mut().intern("Doors of Durin");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: "Whenever you attack" fires once per combat in
                // which you attack with one or more creatures; the
                // closest condition (CreatureAttacks) fires once PER
                // attacking creature.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: scry_and_deploy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn scry_and_deploy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may reveal" optionality (here the reveal is mandatory);
    // a non-creature top card should stay on top but RevealUntil puts it
    // on the bottom; "tapped and attacking" and the conditional
    // until-your-next-turn trample/hexproof grants (Dwarf/Elf checks)
    // are not expressible.
    vec![
        Effect::Scry { player: trig.controller, count: 2 },
        Effect::RevealUntil {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            found_dest: RevealDest::Battlefield,
            rest: DigRest::BottomRandom,
            max_reveal: Some(1),
        },
    ]
}
