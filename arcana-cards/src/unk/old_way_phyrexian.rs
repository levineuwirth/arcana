//! Old Way Phyrexian — `{2}{B}` 2/2 black Phyrexian Cleric with Infect.
//! "Creatures you control with toxic lose toxic and gain infect." is a
//! pure static continuous keyword-swap with no demonstrated primitive,
//! so it is GAP'd; Infect is a base keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Old Way Phyrexian");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Infect],
        ..Default::default()
    };

    // GAP: static "Creatures you control with toxic lose toxic and gain
    // infect" — a continuous keyword-swap static with no demonstrated
    // effect/static primitive.
    reg.register(CardDefinition::new(name, chars))
}
