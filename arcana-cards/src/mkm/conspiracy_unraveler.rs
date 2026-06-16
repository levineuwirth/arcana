//! Conspiracy Unraveler — `{5}{U}{U}` 6/6 Sphinx Detective with Flying.
//! "You may collect evidence 10 rather than pay the mana cost for spells you
//! cast." This is a static alternative-cost permission, not a trigger or
//! activated ability — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conspiracy Unraveler");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);
    subtypes.0.insert(detective);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: "collect evidence 10 rather than pay the mana cost" — static
    //      alternative-cost permission, not modeled.
    reg.register(CardDefinition::new(name, chars))
}
