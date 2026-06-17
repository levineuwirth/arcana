//! Supreme Exemplar — `{6}{U}` 10/10 Elemental with Flying.
//! Champion an Elemental.
//!
//! GAP: Champion is not a usable keyword. Its paired ETB ("sacrifice unless
//! you exile another Elemental you control") and leaves-the-battlefield
//! return are not expressible with the demonstrated primitives.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Supreme Exemplar");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
