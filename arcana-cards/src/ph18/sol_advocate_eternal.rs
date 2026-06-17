//! Sol, Advocate Eternal — `{G}{W}{U}{B}` 4/4 Legendary Dragon Angel.
//! Flying, vigilance.
//! "Legendary partner" (commander-construction static — GAP).
//! "Teamwork — Whenever you attack or block with both Sol, Advocate Eternal and
//! its partner, support 4 and investigate four times." (GAP)
//!
//! Only the Flying + Vigilance keyword line is expressible. Teamwork/Support/
//! Investigate are not modeled keywords/effects, partner identification has no
//! API, and Legendary partner is a deck-construction static.
//!
//! GAP: static — "Legendary partner" (commander-construction rule, no API).
//! GAP: trigger — Teamwork co-attack/block-with-partner → support 4 + investigate
//! four times (no partner predicate; Support and Investigate are not Effect variants).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sol, Advocate Eternal");
    let dragon = reg.interner_mut().intern("Dragon");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
