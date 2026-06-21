//! Werewhat — `{3}{G}` 4/3 Werewolf.
//! Daybound.
//! As Werewhat enters the battlefield, you may exile a creature card from
//! your graveyard or hand. If you do, that card becomes this creature's
//! back face and that back face has nightbound. If you exiled a card from
//! your hand this way, draw a card.
//!
//! Bones only. Daybound is GAP'd (not in the usable keyword surface). The
//! "as enters ... becomes this creature's back face with nightbound" effect
//! is GAP'd: there is no effect to graft an exiled card on as a dynamic back
//! face, and the conditional draw depends on that unmodeled choice.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Werewhat");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Daybound — not in the usable keyword surface.
        ..Default::default()
    };

    // GAP: "As Werewhat enters ... you may exile a creature card ... it becomes
    // this creature's back face with nightbound; if from hand, draw a card."
    // — dynamic-back-face grafting + the dependent draw are unexpressible.
    reg.register(CardDefinition::new(name, chars))
}
