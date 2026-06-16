//! Herald of Slaanesh — `{2}{R}` 2/2 Demon.
//! "Locus of Slaanesh — Demon spells you cast cost {2} less to cast."
//! "Other Demons you control have haste."
//!
//! GAP: "Locus of Slaanesh" is not a supported KeywordAbility variant, and
//! its cost-reduction static has no expressible primitive — omitted.
//! GAP (static): "Other Demons you control have haste" is a pure continuous
//! keyword-granting static with no expressible Effect — omitted.
//! Only the bones are wired.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Herald of Slaanesh");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
