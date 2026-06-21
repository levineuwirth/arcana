//! Mikey & Don, Party Planners — `{2}{G/U}{G/U}` 3/3 Legendary Mutant Ninja
//! Turtle with Ward {2}.
//! "You may look at the top card of your library any time." → GAP (static).
//! "You may play lands and cast Mutant, Ninja, or Turtle spells from the top of
//! your library. If you cast a creature spell this way, that creature enters with
//! an additional +1/+1 counter on it." → GAP (top-of-library play-permission
//! static; no Effect variant).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mikey & Don, Party Planners");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);

    // GAP: "look at the top card any time" and top-of-library play permission.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
