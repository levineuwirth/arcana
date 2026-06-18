//! Mystic Penitent — {W} 1/1 Human Nomad Mystic.
//! Vigilance. Threshold — while 7+ cards are in your graveyard, it gets
//! +1/+1 and has flying.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mystic Penitent");
    let human = reg.interner_mut().intern("Human");
    let nomad = reg.interner_mut().intern("Nomad");
    let mystic = reg.interner_mut().intern("Mystic");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(nomad);
    subtypes.0.insert(mystic);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "Threshold — as long as there are seven or more cards in your
    //      graveyard, this creature gets +1/+1 and has flying." — a
    //      condition-gated static continuous buff; not a triggered/activated
    //      ability and Threshold is not a usable keyword.
    reg.register(CardDefinition::new(name, chars))
}
