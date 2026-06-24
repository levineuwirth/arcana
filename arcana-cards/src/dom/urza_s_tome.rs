//! Urza's Tome — `{2}` artifact — Book.
//! "{3}, {T}: Draw a card. Then discard a card unless you exile a
//! historic card from your graveyard."
//!
//! The draw is wired; the "discard unless you exile a historic card
//! from your graveyard" gate's PAYMENT is exile-a-card-from-graveyard,
//! which is not an `OptionalPaymentKind` (Mana / Life / Sacrifice /
//! Discard only — no exile-from-graveyard cost), so that clause is an
//! honest GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urza's Tome");
    let book = reg.interner_mut().intern("Book");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(book);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{3}, {T}: Draw a card. Then discard a card unless you exile a historic card from your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_then_maybe_discard,
            },
        ),
    )
}

fn draw_then_maybe_discard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'Then discard a card unless you exile a historic card from
    // your graveyard' — the PAYMENT here is exile-a-card-from-graveyard,
    // which is not an OptionalPaymentKind (Mana / Life / Sacrifice /
    // Discard only), so the discard-unless gate is omitted.
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
