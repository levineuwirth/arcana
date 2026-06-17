//! Cid, Timeless Artificer — `{2}{W}{U}` 4/4 Legendary Human Artificer.
//! A filtered dynamic anthem ("Artifact creatures and Heroes you
//! control get +1/+1 for each Artificer you control and each Artificer
//! card in your graveyard"), a deck-construction rule, and Cycling
//! {W}{U}.
//!
//! The anthem is a static continuous effect and is GAP'd; the
//! deck-construction line has no in-game effect. Cycling is wired via
//! the keyword (the engine synthesizes the discard-to-draw activation).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

// GAP: "Artifact creatures and Heroes you control get +1/+1 for each
//      Artificer you control and each Artificer card in your graveyard." —
//      a dynamic static anthem; not a triggered/activated ability.
// GAP: "A deck can have any number of cards named Cid, Timeless Artificer."
//      — deck-construction rule with no in-game effect.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cid, Timeless Artificer");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Cycling(ManaCost::parse("{W}{U}").expect("valid cost"))],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
