//! Wildsear, Scouring Maw — `{3}{R}{G}` 6/6 Legendary Elemental Wolf with
//! Trample.
//!
//! "Enchantment spells you cast from your hand have cascade." is a static
//! continuous ability that grants the Cascade keyword to a class of spells you
//! cast — there is no engine primitive for "spells you cast have <keyword>"
//! (it is neither a triggered nor an activated ability, and no Effect grants a
//! keyword to future cast spells), so it is GAP'd. Only the printed Trample
//! keyword is expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wildsear, Scouring Maw");
    let elemental = reg.interner_mut().intern("Elemental");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(wolf);

    // GAP: static "Enchantment spells you cast from your hand have cascade." —
    // no engine primitive grants a keyword to a class of spells the controller
    // casts; not a triggered/activated ability. Omitted.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
