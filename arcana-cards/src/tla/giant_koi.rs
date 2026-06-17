//! Giant Koi — `{4}{U}{U}` 5/7 Fish.
//! Waterbend {3}: This creature can't be blocked this turn. (Waterbend is an
//! Avatar-set mechanic with no engine support — GAP.)
//! Islandcycling {2} → modeled as generic Cycling {2} per the typecycling rule;
//! the engine synthesizes the discard-to-draw activated ability.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Koi");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        // GAP: Waterbend {3} (can't-be-blocked, Avatar-set mechanic) not modeled.
        // Islandcycling/Landcycling/Typecycling collapse to generic Cycling {2}.
        keywords: vec![KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
