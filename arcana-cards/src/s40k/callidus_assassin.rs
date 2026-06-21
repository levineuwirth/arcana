//! Callidus Assassin — `{4}{U}{B}` 3/3 Human Shapeshifter Assassin.
//!
//! * Flash.
//! * Polymorphine — may enter tapped as a copy of any creature, with an
//!   added destroy-same-name ETB. (Enter-as-a-copy is a copiable-values
//!   replacement, not expressible — GAP.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Callidus Assassin");
    let human = reg.interner_mut().intern("Human");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shapeshifter);
    subtypes.0.insert(assassin);

    // GAP: Polymorphine — "enter as a copy of any creature on the battlefield"
    // is a copy-as-it-enters replacement effect with no Effect primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
