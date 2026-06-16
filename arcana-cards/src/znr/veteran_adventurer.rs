//! Veteran Adventurer — `{5}{G}` 5/5 Human Cleric Rogue Warrior Wizard with
//! Vigilance. Has all four party types; costs {1} less per creature in your
//! party; Vigilance.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Veteran Adventurer");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let rogue = reg.interner_mut().intern("Rogue");
    let warrior = reg.interner_mut().intern("Warrior");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    subtypes.0.insert(rogue);
    subtypes.0.insert(warrior);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static cost reduction "{1} less to cast for each creature in your
    // party" is a cast-time cost modification, not a triggered/activated
    // ability — no expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}
