//! Caller of the Hunt — `{2}{G}` */* Human. "As an additional cost to
//! cast this spell, choose a creature type. Caller of the Hunt's power
//! and toughness are each equal to the number of creatures of the
//! chosen type on the battlefield." The additional-cost type choice and
//! the chosen-type-count CDA are not expressible; only the */* printed
//! P/T is transcribed.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Caller of the Hunt");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // */* printed P/T.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        // GAP: additional cast cost "choose a creature type" is not
        // expressible.
        // GAP: characteristic-defining "P/T equal to the number of
        // creatures of the CHOSEN type on the battlefield" — the count
        // filter depends on a player-chosen subtype, which neither
        // self_pt_from_match (filter fixed at register, no chosen subtype)
        // nor self_pt_cda (no registry/subtype access) can express; the *
        // remains unresolved.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
