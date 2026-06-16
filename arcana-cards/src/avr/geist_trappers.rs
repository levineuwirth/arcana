//! Geist Trappers — `{4}{G}` 3/5 Human Warrior.
//! "Soulbond. As long as this creature is paired with another
//! creature, both creatures have reach."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Geist Trappers");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP (keyword): Soulbond is not in the supported KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };
    // GAP (static): "As long as this creature is paired with another creature,
    // both creatures have reach." — depends on the unmodeled Soulbond pairing;
    // no static/conditional reach-grant-to-pair primitive is available.
    reg.register(CardDefinition::new(name, chars))
}
