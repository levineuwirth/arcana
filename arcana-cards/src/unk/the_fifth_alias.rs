//! The Fifth Alias — `{2}{U}{R}{W}` 5/5 Legendary Creature — Shapeshifter.
//! Pre-game hidden-list / "exclaim its name and cast it free" mechanic.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Fifth Alias");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    // GAP: "Before the game begins, secretly make a list of five nonland cards"
    // — no pre-game / hidden-list machinery exists.
    // GAP: "As you draw a card on your list ... cast it without paying its mana
    // cost" — no draw-trigger keyed to a secret list, and no free-cast-from-list
    // effect; entirely unexpressible.
    reg.register(CardDefinition::new(name, chars))
}
