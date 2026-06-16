//! Vine Dryad — `{3}{G}` 1/3 Dryad with Flash and Forestwalk.
//!
//! "You may exile a green card from your hand rather than pay this
//! spell's mana cost. Flash. Forestwalk."
//!
//! Flash is a base keyword; Forestwalk maps to Landwalk("Forest"). The
//! alternative cost (exile a green card from hand instead of paying mana)
//! is GAP'd — not a demonstrated keyword/ability primitive.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vine Dryad");
    let dryad = reg.interner_mut().intern("Dryad");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Landwalk(forest)],
        ..Default::default()
    };

    // GAP: "You may exile a green card from your hand rather than pay
    //       this spell's mana cost" — alternative cost, no primitive.
    reg.register(CardDefinition::new(name, chars))
}
