//! Osteomancer Adept — `{1}{B}` 2/2 black Squirrel Warlock with Deathtouch.
//!
//! * Deathtouch → base keyword.
//! * "{T}: Until end of turn, you may cast creature spells from your
//!   graveyard by foraging … (finality counter rider)." → a `{T}`
//!   activated ability whose payload is GAP'd: granting a temporary
//!   alternative-cost cast permission from the graveyard (forage cost,
//!   finality-counter-on-entry rider) is not expressible with the
//!   demonstrated Effect catalog.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Osteomancer Adept");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Until end of turn, you may cast creature spells from your graveyard by foraging in addition to paying their other costs. If you cast a spell this way, that creature enters with a finality counter on it.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: cast_from_graveyard_grant,
        }),
    )
}

fn cast_from_graveyard_grant(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granting a temporary alternative-cost (forage) permission to
    // cast creature spells from the graveyard, plus the finality-counter
    // entry rider, is not expressible with the demonstrated Effect catalog.
    Vec::new()
}
