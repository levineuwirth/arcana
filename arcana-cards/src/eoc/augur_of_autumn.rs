//! Augur of Autumn — `{1}{G}{G}` 2/3 Human Druid.
//! "You may look at the top card of your library any time."
//! "You may play lands from the top of your library."
//! "Coven — As long as you control three or more creatures with different
//! powers, you may cast creature spells from the top of your library."
//!
//! All three lines are static "play/cast from the top of your library"
//! permissions (the third gated by Coven). There is no play-from-top /
//! cast-from-top static primitive in the demonstrated API, and Coven is not
//! a usable KeywordAbility variant — so all three abilities are GAP'd and
//! only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Augur of Autumn");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    // GAP: "You may look at the top card of your library any time."
    // GAP: "You may play lands from the top of your library."
    // GAP: Coven — "you may cast creature spells from the top of your library."
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
