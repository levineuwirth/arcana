//! Kruphix, God of Horizons — `{3}{G}{U}` 4/7 Legendary Enchantment Creature — God.
//! Indestructible.
//! As long as your devotion to green and blue is less than seven, Kruphix isn't a creature.
//! You have no maximum hand size.
//! If you would lose unspent mana, that mana becomes colorless instead.
//!
//! Decomposition:
//! 1. Keyword line: Indestructible.
//! 2-4. The remaining three lines are PURE STATIC abilities (no trigger word,
//!    no activation cost):
//!      • devotion-gated "isn't a creature" — GAP (no static-characteristic
//!        Effect surface on this card class).
//!      • "no maximum hand size" — GAP (static, no Effect).
//!      • "unspent mana becomes colorless" — GAP (static replacement, no Effect).
//!    All three are static-only; this MultiAbilityCreature class carries only
//!    keyword / triggered / activated abilities, so the statics are documented
//!    GAPs and only the keyword + bones are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kruphix, God of Horizons");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
