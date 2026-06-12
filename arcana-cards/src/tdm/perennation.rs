//! Perennation — `{3}{W}{B}{G}` sorcery. "Return target permanent
//! card from your graveyard to the battlefield with a hexproof counter
//! and an indestructible counter on it." The hexproof counter rides the
//! return as `CounterKind::Named("hexproof")` via
//! `ReturnFromGraveyardWithCounters`. GAP: that primitive carries a
//! single counter kind, so the indestructible counter (and the keyword
//! grants from both keyword counters) are not expressible — the zone
//! move re-ids the object, blocking card-side follow-ups.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Perennation");
    // Interned for the effect fn's lookup of the named counter kind.
    reg.interner_mut().intern("hexproof");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target permanent card from your graveyard to the battlefield with a hexproof counter and an indestructible counter on it.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::permanent(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let _ = entry.controller;
    let Some(kind) = reg.interner().lookup("hexproof").map(CounterKind::Named) else {
        return vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }];
    };
    // GAP: ReturnFromGraveyardWithCounters carries one counter kind — the
    // indestructible counter and both keyword grants (CR 122.1g) are not
    // expressible (the zone move re-ids the object).
    vec![Effect::ReturnFromGraveyardWithCounters {
        target: *id,
        kind,
        count: 1,
    }]
}
