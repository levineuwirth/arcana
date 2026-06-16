//! Skill Borrower — `{2}{U}` 1/3 Artifact Creature — Human Wizard.
//! Play with the top card of your library revealed (static — GAP).
//! As long as the top card is an artifact or creature card, this creature has
//! all activated abilities of that card (static ability-grant — GAP).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skill Borrower");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    // GAP (static): "Play with the top card of your library revealed" — no
    //   library-reveal static is expressible.
    // GAP (static): "has all activated abilities of [the top card]" — dynamic
    //   ability-grant from another card is not expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
