//! Ardyn, the Usurper — `{5}{B}{B}{B}` 4/4 legendary black Elder Human
//! Noble.
//!
//! Oracle:
//! * Demons you control have menace, lifelink, and haste. (static — GAP'd)
//! * Starscourge — At the beginning of combat on your turn, exile up to
//!   one target creature card from a graveyard. If you exiled a card this
//!   way, create a token that's a copy of that card, except it's a 5/5
//!   black Demon. (GAP'd)
//!
//! "Starscourge" is an ability word, not a keyword. The Demon static is a
//! continuous keyword-granting effect (GAP'd). The combat trigger exiles
//! a graveyard CARD and mints a copy-token with overridden P/T, color,
//! and subtype; `Effect::CopyPermanent` copies a battlefield permanent
//! verbatim with no modification and cannot copy a graveyard card, so the
//! ability is GAP'd rather than approximated.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ardyn, the Usurper");
    let elder = reg.interner_mut().intern("Elder");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(human);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "Demons you control have menace, lifelink, and haste."
    // GAP: trigger — exile a graveyard creature card, then create a copy
    // token "except it's a 5/5 black Demon." No copy-with-overrides of a
    // graveyard card primitive.

    reg.register(CardDefinition::new(name, chars))
}
