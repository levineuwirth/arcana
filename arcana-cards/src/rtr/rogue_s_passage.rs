//! Rogue's Passage — nonbasic land (Return to Ravnica, 2012).
//! "{T}: Add {C}." and "{4}, {T}: Target creature can't be blocked
//! this turn." The seed touchstone for the UtilityLand card-gen
//! shape: a land is a `Characteristics` with `mana_cost: None`,
//! `colors: ColorSet::new()`, `TypeLine::LAND`, plus activated
//! abilities exactly like a creature's — a mana ability and, here,
//! one utility activation.
//!
//! # Rules references
//!
//! * CR 305.1 — lands are played, not cast; the engine's land-drop
//!   path keys purely on `TypeLine::LAND` (no `SpellAbilityDef`).
//! * CR 605.1a — "{T}: Add {C}" is a mana ability
//!   (`is_mana_ability: true`, skips the stack).
//! * CR 509.1b — "can't be blocked" is a blocking restriction;
//!   [`Effect::CantBeBlocked`] installs the layer-system restriction
//!   until end of turn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rogue's Passage");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}, {T}: Target creature can't be blocked this turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: unblockable,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn unblockable(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::CantBeBlocked { target: *id, duration: Duration::EndOfTurn }]
}
