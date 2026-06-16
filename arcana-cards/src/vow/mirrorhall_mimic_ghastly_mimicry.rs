//! Mirrorhall Mimic // Ghastly Mimicry — {3}{U} Creature — Spirit // Enchantment — Aura
//!
//! Front (Mirrorhall Mimic) 0/0:
//!   You may have this creature enter as a copy of any creature on the battlefield, except it's a
//!   Spirit in addition to its other types.
//!   GAP: "you may have this enter as a copy" replacement ETB effect not expressible.
//!   Disturb {3}{U}{U} — GAP: Disturb casting mechanic not modeled.
//!
//! Back (Ghastly Mimicry): Enchantment — Aura
//!   Enchant creature
//!   At the beginning of your upkeep, create a token that's a copy of enchanted creature, except
//!   it's a Spirit in addition to its other types.
//!   GAP: back-face-only triggered ability (upkeep copy token) not modeled.
//!   GAP: CopyPermanent with Spirit-addition modification not expressible.
//!   If Ghastly Mimicry would be put into a graveyard from anywhere, exile it instead.
//!   GAP: replacement effect for graveyard → exile not modeled.
//!
//! Keyword note: Disturb, Enchant, Transform are not KeywordAbility variants; keywords: vec![].

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mirrorhall Mimic");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: Disturb keyword not in engine keyword list
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ghastly Mimicry");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
        // GAP: "You may have this enter as a copy" ETB replacement effect not modeled
        // GAP: back-face-only triggered ability (upkeep: create copy token as Spirit) not modeled
        // GAP: "if Ghastly Mimicry would be put into a graveyard, exile instead" not modeled
    )
}
