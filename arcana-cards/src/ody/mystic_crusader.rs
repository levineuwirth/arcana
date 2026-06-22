//! Mystic Crusader — `{1}{W}{W}` 2/1 Human Nomad Mystic.
//! Protection from black and from red; Threshold — as long as there are
//! seven or more cards in your graveyard, this creature gets +1/+1 and
//! has flying.
//!
//! GAP: Protection (from black, from red) is not in the supported
//! KeywordAbility surface — emit `keywords: vec![]`.
//! GAP: Threshold static — "as long as there are seven or more cards in
//! your graveyard, ~ gets +1/+1 and has flying" is a conditional static
//! continuous ability with no trigger/cost. Not expressible as a
//! triggered/activated ability; omitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mystic Crusader");
    let human = reg.interner_mut().intern("Human");
    let nomad = reg.interner_mut().intern("Nomad");
    let mystic = reg.interner_mut().intern("Mystic");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(nomad);
    subtypes.0.insert(mystic);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
