//! Mm'menon, the Right Hand — `{3}{U}{U}` 3/4 Legendary Jellyfish Advisor
//! with Flying.
//! You may look at the top card of your library any time (static — GAP).
//! You may cast artifact spells from the top of your library (static — GAP).
//! Artifacts you control have "{T}: Add {U}. Spend this mana only to cast a
//!   spell from anywhere other than your hand." (static ability-grant — GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mm'menon, the Right Hand");
    let jellyfish = reg.interner_mut().intern("Jellyfish");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jellyfish);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: all three non-keyword lines are static abilities (top-of-library
    // peek, cast-from-top permission, and granting an activated mana ability to
    // your artifacts) — none expressible as triggered/activated abilities.

    reg.register(CardDefinition::new(name, chars))
}
