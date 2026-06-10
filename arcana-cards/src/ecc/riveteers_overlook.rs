//! Riveteers Overlook — nonbasic land (Streets of New Capenna, 2022).
//! "When this land enters, sacrifice it. When you do, search your
//! library for a basic Swamp, Mountain, or Forest card, put it onto
//! the battlefield tapped, then shuffle and you gain 1 life." The
//! reflexive trigger is modeled as one ETB sequence: sacrifice this
//! land (by name filter), fetch a matching basic tapped, gain 1.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Riveteers Overlook");
    // Pre-intern the fetchable basic subtypes for resolver lookups.
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: sac_and_fetch,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn sac_and_fetch(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let self_filter = ObjectFilter {
        name: reg.interner().lookup("Riveteers Overlook"),
        ..ObjectFilter::default()
    };
    let subtypes: Vec<_> = ["Swamp", "Mountain", "Forest"]
        .iter()
        .filter_map(|s| reg.interner().lookup(s))
        .collect();
    let fetch_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
        .with_subtypes_any(subtypes);
    vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: self_filter,
            count: 1,
        },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: fetch_filter,
            tapped: true,
        },
        Effect::GainLife { player: trig.controller, amount: 1 },
    ]
}
