//! Hostile Hostel // Creeping Inn — transforming DFC (CR 712).
//!
//! Front face (Hostile Hostel): Land.
//!   {T}: Add {C}.
//!   {1}, {T}, Sacrifice a creature: Put a soul counter on this land. Then if there are
//!   three or more soul counters on it, remove those counters, transform it, then untap it.
//!   Activate only as a sorcery.
//!
//! Back face (Creeping Inn): Artifact Creature — Horror Construct.
//!   Whenever this creature attacks, you may exile a creature card from your graveyard.
//!   If you do, each opponent loses X life and you gain X life, where X is the number of
//!   creature cards exiled with this creature.
//!   {4}: This creature phases out.
//!
//! # GAPs
//! - Front-face activated abilities ({T}: Add {C}; the {1},{T},Sac-a-creature soul-counter
//!   accrual + self-transform; sorcery-speed restriction) are not expressible with the
//!   documented triggered/transform/effect API surface — no activated-ability shape, no
//!   counter-threshold activation cost. Front face emitted as a bare Land.
//! - Back-face attack trigger uses a dynamic X = "creature cards exiled with this creature",
//!   a persistent per-permanent tally not available via the documented `script::*` helpers,
//!   and the "may exile a creature card from your graveyard" cost is also not expressible.
//!   Emitted as a no-op trigger body rather than a wrong literal.
//! - Back-face "{4}: This creature phases out" activated ability is not expressible (no
//!   activated-ability / phase-out effect in the documented surface).

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hostile Hostel");

    // Front face: a colorless Land.
    let chars = Characteristics {
        name,
        colors: ColorSet::colorless(),
        types: TypeLine::LAND.into(),
        // GAP: {T}: Add {C}; and {1},{T},Sac a creature: soul-counter accrual + self-transform
        // (sorcery-speed) — activated abilities not expressible with the documented API.
        ..Default::default()
    };

    // Back face: Creeping Inn — colorless Artifact Creature — Horror Construct, 5/8.
    let back_name = reg.interner_mut().intern("Creeping Inn");
    let horror_sub = reg.interner_mut().intern("Horror");
    let construct_sub = reg.interner_mut().intern("Construct");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(horror_sub);
    back_subtypes.0.insert(construct_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(8)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back-face only: "Whenever this creature attacks, ..." (id 1, gated to face 1).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1),
    )
}

fn attack_drain(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may exile a creature card from your graveyard; if you do, each opponent loses
    // X life and you gain X, where X = creature cards exiled with this creature." Both the
    // optional graveyard-exile cost and the persistent per-permanent exile tally (dynamic X)
    // are outside the documented effect/script surface. An honest no-op beats a wrong literal.
    let _ = ControllerConstraint::You;
    Vec::new()
}
