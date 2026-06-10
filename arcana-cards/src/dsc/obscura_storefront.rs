//! Obscura Storefront — nonbasic land.
//! "When this land enters, sacrifice it. When you do, search your
//! library for a basic Plains, Island, or Swamp card, put it onto the
//! battlefield tapped, then shuffle and you gain 1 life." Modeled as
//! a single ETB trigger that sacrifices the land (via a name-filtered
//! `Effect::Sacrifice` — fidelity note: a same-named copy could be
//! chosen) and resolves the reflexive search + life gain inline.

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
    let name = reg.interner_mut().intern("Obscura Storefront");
    // Pre-intern the basic land subtypes for the resolver's read-only
    // lookup.
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars).with_triggered_ability(
        TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: sac_and_fetch,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        },
    ))
}

fn sac_and_fetch(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Sacrifice this land — Effect::Sacrifice has no self-id form, so the
    // sacrifice is filtered by this card's name (fidelity note: another
    // permanent named Obscura Storefront you control could be chosen).
    let nm = reg.interner().lookup("Obscura Storefront");
    let mut syms = Vec::new();
    if let Some(s) = reg.interner().lookup("Plains") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Island") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Swamp") {
        syms.push(s);
    }
    vec![Effect::Sequence(vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter {
                name: nm,
                types: Some(TypeLine::LAND.into()),
                ..ObjectFilter::default()
            },
            count: 1,
        },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
                .with_subtypes_any(syms),
            tapped: true,
        },
        Effect::GainLife { player: trig.controller, amount: 1 },
    ])]
}
