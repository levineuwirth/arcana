//! Ulna Alley Shopkeep — `{2}{B}` 2/3 Goblin Warlock (B) with Menace.
//! Infusion is a mechanic word, not a usable KeywordAbility. The "+2/+0 as long as
//! you gained life this turn" body is a conditional continuous static, not a
//! triggered/activated ability — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ulna Alley Shopkeep");
    let goblin = reg.interner_mut().intern("Goblin");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warlock);

    // GAP: "Infusion — This creature gets +2/+0 as long as you gained life this turn."
    // Conditional continuous P/T static, not a triggered/activated ability.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
