//! Chronomancer — `{1}{B}` 1/1 Artifact Creature — Necron Wizard. Flying.
//! "Atomic Transmutation — {1}, {T}, Sacrifice another artifact: Draw a card."
//! "Unearth {2}{B}."
//!
//! Flying is a base keyword. "Atomic Transmutation" is an ability-word label,
//! dropped; the ability itself is a mana+tap+sacrifice-an-artifact activation
//! that draws a card. Unearth is NOT in the usable keyword surface, and its
//! full mechanic (return from graveyard, gain haste, exile at the next end
//! step) has no expressible primitive here, so Unearth is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chronomancer");
    let necron = reg.interner_mut().intern("Necron");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);
    subtypes.0.insert(wizard);

    // GAP: Unearth {2}{B} — not in the usable keyword surface; the return-from-
    // graveyard + haste + exile-at-end-step mechanic has no expressible
    // primitive for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, {T}, Sacrifice another artifact: Draw a card.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                tap: true,
                sacrifice_other: Some(
                    ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                ),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_one,
        }),
    )
}

fn draw_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
