//! Chitinous Crawler — `{2}{B}{B}` 4/4 Creature — Insect Horror.
//! "At the beginning of combat on your turn, choose target creature card in
//!  your graveyard. Conjure a duplicate of it into your graveyard."
//! "Descend 8 — Exile a permanent card from your graveyard: You may play it.
//!  Activate only as a sorcery and only if there are eight or more permanent
//!  cards in your graveyard."
//!
//! Decomposition:
//! - Keyword line: Scryfall tags "Descend" and "Conjure" — neither is in the
//!   usable keyword surface, so `keywords: vec![]`.
//! - "At the beginning of combat on your turn, choose target creature card in
//!   your graveyard, …" → a PhaseBegins(Combat, You) triggered ability that
//!   targets a creature card in your graveyard.
//!   GAP: "Conjure a duplicate of it into your graveyard" — Conjure is an
//!   Arena-only mechanic with no Effect variant, so the effect is empty.
//! - GAP (whole ability): the Descend-8 graveyard activated ability — its cost
//!   "Exile a permanent card from your graveyard" has no ActivationCost field,
//!   and the "you may play it" impulse + Descend-8 gating are not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chitinous Crawler");
    let insect = reg.interner_mut().intern("Insect");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: conjure_duplicate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

/// GAP: "Conjure a duplicate of it into your graveyard" — Conjure (Arena-only)
/// has no Effect variant.
fn conjure_duplicate(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
