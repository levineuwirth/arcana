//! Jace, Vryn's Prodigy // Jace, Telepath Unbound — `{1}{U}` Legendary Creature
//! — Human Wizard 0/2 (front). Activated ability: `{T}: Draw a card, then
//! discard a card. If there are five or more cards in your graveyard, exile
//! Jace, then return him to the battlefield transformed under his owner's
//! control.`
//!
//! Back face: Legendary Planeswalker — Jace (loyalty 5).
//! +1: Up to one target creature gets -2/-0 until your next turn.
//! -3: You may cast target instant or sorcery card from your graveyard this
//!     turn. If that spell would be put into your graveyard, exile it instead.
//! -9: You get an emblem with "Whenever you cast a spell, target opponent mills
//!     five cards."
//!
//! # GAPs
//! - The front-face activated ability (`{T}: draw then discard; if 5+ cards in
//!   graveyard, exile then return transformed`) requires checking graveyard size
//!   at activation time and then exiling + returning from exile as a transform
//!   sequence. ActivatedAbilityDef is not in this prompt's API surface; the
//!   triggered-ability approach is used instead as a best-effort stub, but the
//!   exact condition (`graveyard_size >= 5` check before transform) and the
//!   tap/draw/discard part cannot be fully modeled. The ability is omitted.
//! - Planeswalker loyalty abilities (+1/-3/-9) are not expressible: the engine
//!   has no LoyaltyAbilityDef in the demonstrated API.
//! - "You may cast target instant or sorcery card from your graveyard this
//!   turn" — casting from graveyard permission not expressible.
//! - Emblem creation not available.
//! Back-face-only triggered abilities are not modeled (GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::PendingTrigger;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace, Vryn's Prodigy");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Legendary Planeswalker — Jace with starting loyalty 5.
    let back_name = reg.interner_mut().intern("Jace, Telepath Unbound");
    let jace_sub = reg.interner_mut().intern("Jace");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(jace_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(5),
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: front-face tap activated ability not modeled (ActivatedAbilityDef
    // not in demonstrated API for this prompt shape; graveyard-size conditional
    // transform + exile/return sequence not expressible).
    // GAP: planeswalker loyalty abilities (+1/-3/-9) not modeled.
    // GAP: back-face-only triggered abilities not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
