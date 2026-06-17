//! Alhammarret, High Arbiter — `{5}{U}{U}` 5/5 Legendary Sphinx with Flying.
//! ETB: each opponent reveals their hand; you choose the name of a nonland
//! card revealed. Static: your opponents can't cast spells with the chosen
//! name. Both the name-choice ETB and the resulting cast-restriction are
//! GAP'd — no name-choosing/cast-locking Effect in the catalog.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alhammarret, High Arbiter");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: "As Alhammarret enters, each opponent reveals their hand. You choose
    // the name of a nonland card revealed this way" + "Your opponents can't
    // cast spells with the chosen name" — no name-choice / cast-prohibition
    // Effect primitive; only Flying is expressible.
    reg.register(CardDefinition::new(name, chars))
}
