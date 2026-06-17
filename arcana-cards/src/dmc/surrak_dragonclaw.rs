//! Surrak Dragonclaw — `{2}{G}{U}{R}` 6/6 Legendary Human Warrior with Flash.
//! "This spell can't be countered." (static — GAP)
//! "Creature spells you control can't be countered." (static — GAP)
//! "Other creatures you control have trample." (static anthem — GAP)
//!
//! All non-keyword text is static continuous abilities, none expressible as a
//! triggered/activated ability. Only the Flash keyword is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surrak Dragonclaw");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
