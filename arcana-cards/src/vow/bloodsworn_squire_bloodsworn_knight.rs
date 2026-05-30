//! Bloodsworn Squire // Bloodsworn Knight — `{3}{B}` Vampire Soldier creature 3/3.
//! Front: {1}{B}, Discard a card: This creature gains indestructible until end of
//!   turn. Tap it. Then if there are four or more creature cards in your graveyard,
//!   transform this creature.
//! Back (Bloodsworn Knight): P/T equal to number of creature cards in graveyard.
//!   {1}{B}, Discard a card: This creature gains indestructible until end of turn.
//!   Tap it.
//!
//! GAP: Activated ability costs include "Discard a card" — discard as a cost is
//!   not expressible via OptionalPaymentKind (only Mana and Life supported).
//!   The activation and its conditional transform effect are not modeled.
//! GAP: Back face P/T is dynamic ("equal to number of creature cards in your
//!   graveyard") — dynamic P/T not supported on Characteristics; fixed 0/0 used.
//! GAP: Back face's own activated ability not modeled (back-face-only triggered
//!   ability not auto-installed on transform).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodsworn Squire");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);
    subtypes.0.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Bloodsworn Knight");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vampire_sub);
    back_subtypes.0.insert(knight_sub);

    // GAP: back face P/T is dynamic (= number of creature cards in graveyard).
    // Using fixed 0/0 as a placeholder; the engine cannot express dynamic P/T here.
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: {1}{B}, Discard a card: gain indestructible + tap + conditional transform.
    // Discard-a-card cost is not expressible (OptionalPaymentKind supports only
    // Mana and Life). The activation is omitted entirely.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
