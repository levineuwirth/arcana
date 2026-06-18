//! Stratadon — `{10}` 5/5 Artifact Creature — Beast with Trample.
//! "Domain — This spell costs {1} less to cast for each basic land type
//! among lands you control."
//!
//! Trample is a base keyword. Domain is a cast-time cost reduction with
//! no expressible primitive (a static cost modifier on the spell itself);
//! GAP'd — only the bones + Trample are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stratadon");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    // GAP: Domain — cast-time cost reduction static; no expressible primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
