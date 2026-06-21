//! Eligeth, Crossroads Augur — `{4}{U}{U}` 5/6 Legendary Sphinx with
//! Flying. "If you would scry a number of cards, draw that many cards
//! instead." "Partner."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eligeth, Crossroads Augur");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    // GAP: static replacement "If you would scry a number of cards, draw that
    // many cards instead." — a scry→draw replacement static is not
    // expressible with the available triggered/activated primitives.
    // GAP: "Partner" — not a supported KeywordAbility variant (Commander
    // partner pairing is not modeled).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
