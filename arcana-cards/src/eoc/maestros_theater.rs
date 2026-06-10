//! Maestros Theater — nonbasic land. "When this land enters, sacrifice
//! it. When you do, search your library for a basic Island, Swamp, or
//! Mountain card, put it onto the battlefield tapped, then shuffle and
//! you gain 1 life."
//!
//! Modeled as a single ETB trigger: sacrifice this land (via a
//! name-filtered `Effect::Sacrifice` — the closest sacrifice-self
//! primitive), then fetch + gain 1 life. The reflexive "when you do"
//! split is flattened into one resolution.

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
    let name = reg.interner_mut().intern("Maestros Theater");
    // Pre-intern basic land subtypes for the resolver's read-only lookups.
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
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
                effect: etb_sac_and_fetch,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn etb_sac_and_fetch(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Sacrifice this land: no sacrifice-self Effect exists, so use a
    // name-filtered Sacrifice (matches this land).
    let nm = reg.interner().lookup("Maestros Theater");
    let mut subs = Vec::new();
    for n in ["Island", "Swamp", "Mountain"] {
        if let Some(s) = reg.interner().lookup(n) {
            subs.push(s);
        }
    }
    let fetch_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
        .with_subtypes_any(subs);
    vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
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
