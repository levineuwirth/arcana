//! Sphinx of the Final Word — `{5}{U}{U}` 5/5 Sphinx with Flying and Hexproof.
//! This spell can't be countered. Instant and sorcery spells you control
//! can't be countered.
//!
//! Both can't-be-countered clauses are static abilities with no
//! triggered/activated representation — only Flying and Hexproof are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sphinx of the Final Word");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    // GAP: "This spell can't be countered" — static, not a triggered/activated ability.
    // GAP: "Instant and sorcery spells you control can't be countered" — static.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
