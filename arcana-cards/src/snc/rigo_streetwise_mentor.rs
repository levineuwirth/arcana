//! Rigo, Streetwise Mentor — `{G/W}{W}{W/U}` 2/2 Legendary Cat Citizen (G/U/W).
//!
//! * "Rigo enters with a shield counter on it." → an ETB trigger that
//!   puts a `CounterKind::Shield` counter on Rigo (`trig.source`).
//! * "Whenever you attack a player or planeswalker with one or more
//!   creatures with power 1 or less, draw a card." → modeled as a
//!   `CreatureAttacks` trigger filtered to creatures you control with
//!   power 1 or less; on fire, the controller draws a card.
//!   FIDELITY GAP: the printed ability is a single batched
//!   "whenever you attack … with one or more such creatures" trigger
//!   (draws exactly one card per combat); this fires once per
//!   qualifying attacker. No batched-attack trigger primitive is
//!   available in the demonstrated API.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rigo, Streetwise Mentor");
    let cat = reg.interner_mut().intern("Cat");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G/W}{W}{W/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enters_with_shield,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_max_power(1),
                },
                intervening_if: None,
                effect: draw_on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn enters_with_shield(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Shield,
        count: 1,
    }]
}

fn draw_on_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
