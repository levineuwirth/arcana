//! Vexilus Praetor — `{3}{W}` 3/4 Custodes Warrior with Flash and
//! Vigilance.
//!
//! Aegis of the Emperor — Commanders you control have protection from
//! everything. This is a static continuous ability granting protection
//! to a Commander-typed subset of your creatures; there is no expressible
//! primitive for it in the multi-ability creature surface (no static
//! "creatures matching filter have protection" effect, and protection
//! is not a usable KeywordAbility), so it is GAP'd. Flash and Vigilance
//! are base keywords.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vexilus Praetor");
    let custodes = reg.interner_mut().intern("Custodes");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(custodes);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "Commanders you control have protection from everything"
    // — no expressible static-protection-grant primitive in this surface.
    reg.register(CardDefinition::new(name, chars))
}
