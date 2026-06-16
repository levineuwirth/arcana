//! Diregraf Escort — `{G}` 1/1 Human Cleric.
//! "Soulbond" (not in usable keyword surface — GAP).
//! "As long as this creature is paired with another creature, both creatures have
//! protection from Zombies." (soulbond-pairing static — GAP).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Diregraf Escort");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: Soulbond keyword (not in usable surface) + the soulbond-pairing static
    // "both creatures have protection from Zombies" — neither is expressible.
    reg.register(CardDefinition::new(name, chars))
}
