//! Tranquil Thicket — Onslaught common cycling land. "Tranquil
//! Thicket enters the battlefield tapped. {T}: Add {G}. Cycling
//! {2}." Seed touchstone for CR 702.29 Cycling: a card with an
//! activated ability that lives in *hand* alongside one that lives
//! on the battlefield — the machinery has to route each activation
//! to the right pipeline based on the source's current zone.
//!
//! # Rules references
//!
//! * CR 702.29a — Cycling `[cost]` is an activated ability "pay
//!   [cost], discard this card: draw a card." Activatable at
//!   instant speed (CR 702.29a).
//! * CR 113.6 — An activated ability is activatable only while its
//!   card is in the zone the ability specifies. Cycling lives in
//!   Hand; the mana ability lives on Battlefield. Both are modeled
//!   as [`arcana_core::registry::ActivatedAbilityDef`]s with
//!   distinct [`arcana_core::registry::ActivationZone`]s.
//! * CR 400.7 — Moving the card from Hand to Graveyard re-ids the
//!   object. The cycling ability's resolution reads the ability
//!   definition via the stack-entry-snapshotted `card_id`, so the
//!   re-id is transparent to the draw effect.
//!
//! # Deliberate simplification
//!
//! The "enters the battlefield tapped" clause is not modeled yet —
//! the ETB-tapped mechanic lands as part of a later seed. For the
//! cycling-seed scope, a Thicket that enters untapped still cycles
//! and still taps for green, which is all the integration tests
//! exercise.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tranquil Thicket");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    // Tranquil Thicket doesn't have the Forest subtype in its oracle
    // text (it's a non-basic land without a basic-land subtype). The
    // name "Forest" here is a leftover interner cue for grouping;
    // keeping the subtypes empty to match the printed card.
    let _ = forest;
    subtypes.0.clear();

    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,

                face_gate: None,
                effect: add_green_mana,
            })
            .with_cycling(ManaCost::parse("{2}").expect("valid cycling cost")),
    )
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}
