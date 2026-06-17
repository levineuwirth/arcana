//! Catacomb Sifter — `{1}{B}{G}` 2/3 Eldrazi Drone (Devoid → colorless).
//! "When this creature enters, create a 1/1 colorless Eldrazi Scion creature
//!  token (with 'Sacrifice this token: Add {C}.' — GAP'd inner ability).
//!  Whenever another creature you control dies, scry 1."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Catacomb Sifter");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    reg.interner_mut().intern("Scion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        // Devoid: this card has no color.
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_scion,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // "Whenever another creature you control dies" — modeled as a
                // creature you control moving to the graveyard (the "another"
                // self-exclusion is a fidelity gap).
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: scry1,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_scion(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let scion = reg.interner().lookup("Scion").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(reg.interner().lookup("Eldrazi").unwrap_or_default());
    subtypes.0.insert(scion);
    // GAP: the token's "Sacrifice this token: Add {C}." inner activated ability
    // is not expressible via TokenDefinition.abilities in this card class.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: scion,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn scry1(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry { player: trig.controller, count: 1 }]
}
