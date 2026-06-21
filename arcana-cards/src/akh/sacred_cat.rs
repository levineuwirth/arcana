//! Sacred Cat — `{W}` 1/1 Cat with Lifelink.
//!
//! Oracle:
//! * Lifelink
//! * Embalm {W} ({W}, Exile this card from your graveyard: Create a token
//!   that's a copy of it, except it's a white Zombie Cat with no mana cost.
//!   Embalm only as a sorcery.)
//!
//! Lifelink is a base keyword. Embalm is not a usable keyword variant, and its
//! graveyard activation ("create a token that's a copy of this card") is not
//! expressible — CopyPermanent copies a battlefield permanent, not a graveyard
//! card. Following established precedent (Unwavering Initiate et al.), Embalm
//! is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sacred Cat");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: Embalm {W} — not a usable keyword; the graveyard copy-token
    // activation is not expressible with the available primitives.
    reg.register(CardDefinition::new(name, chars))
}
