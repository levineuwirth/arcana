//! Splinter, Hamato Yoshi — `{1}{B}` 1/3 Legendary Mutant Ninja Rat.
//! Sneak {B}; Menace; "Other Ninjas you control get +1/+1."
//!
//! Keyword line: Menace (a fully-implemented evergreen keyword). Sneak
//! is an alternative-cast keyword not in the usable KeywordAbility
//! surface — GAP. The "+1/+1 to other Ninjas" line is a pure static
//! continuous anthem, which is not a triggered/activated ability and
//! cannot be expressed in this card shape — GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Splinter, Hamato Yoshi");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(rat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword — Sneak {B} (alternative-cast keyword; no KeywordAbility variant).
        keywords: vec![KeywordAbility::Menace],
        // GAP: static — "Other Ninjas you control get +1/+1" (continuous anthem, not a triggered/activated ability).
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
