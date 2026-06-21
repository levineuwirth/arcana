//! Doc Ock, Sinister Scientist — `{4}{U}` 4/5 Legendary Human Scientist Villain.
//! As long as there are eight or more cards in your graveyard, Doc Ock
//!   has base power and toughness 8/8.
//! As long as you control another Villain, Doc Ock has hexproof.
//!
//! Both clauses are conditional static continuous abilities (no trigger
//! word, no cost). Neither is expressible as a triggered/activated
//! ability in this card class — conditional base-P/T setting and
//! conditional keyword grants are statics. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doc Ock, Sinister Scientist");
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scientist);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP (static): "As long as there are eight or more cards in your
    //   graveyard, Doc Ock has base power and toughness 8/8."
    // GAP (static): "As long as you control another Villain, Doc Ock has
    //   hexproof."
    reg.register(CardDefinition::new(name, chars))
}
