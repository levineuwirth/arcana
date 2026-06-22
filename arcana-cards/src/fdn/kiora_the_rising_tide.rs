//! Kiora, the Rising Tide — `{2}{U}` 3/2 legendary Merfolk Noble.
//! "When Kiora enters, draw two cards, then discard two cards."
//! "Threshold — Whenever Kiora attacks, if there are seven or more cards in
//!  your graveyard, you may create Scion of the Deep, a legendary 8/8 blue
//!  Octopus creature token."

use arcana_core::conditions;
use arcana_core::effects::{DiscardChoice, Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kiora, the Rising Tide");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_loot_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: Some(if_threshold),
                effect: make_scion,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_loot_two(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 2,
        },
        Effect::Discard {
            player: trig.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn if_threshold(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::graveyard_at_least(s, you, 7)
}

fn make_scion(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let name = reg
        .interner()
        .lookup("Scion of the Deep")
        .unwrap_or_default();
    let octopus = reg.interner().lookup("Octopus").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(octopus);
    // "you may create" — the engine treats token creation as performed;
    // the optional "may" is a resolution-time choice we cannot gate here,
    // so it is created unconditionally (a documented may-simplification).
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(8)),
            toughness: Some(PtValue::Fixed(8)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
