//! Quintorius, Field Historian — `{3}{R}{W}` 2/4 Legendary Elephant Cleric.
//! "Spirits you control get +1/+0. (static — GAP'd)
//!  Whenever one or more cards leave your graveyard, create a 3/2 red and
//!  white Spirit creature token."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quintorius, Field Historian");
    let elephant = reg.interner_mut().intern("Elephant");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Spirits you control get +1/+0" is a continuous anthem, not
    // a triggered/activated ability — not expressible in this card class.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "Whenever one or more cards leave your graveyard" — modeled
                // as a card leaving the graveyard zone (to any zone).
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: arcana_core::targets::ObjectFilter::default()
                        .controlled_by(arcana_core::targets::ControllerConstraint::You),
                    from: Some(Zone::Graveyard(0)),
                    to: Zone::Exile,
                },
                intervening_if: None,
                effect: make_spirit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_spirit(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spirit,
            colors: ColorSet::red() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
