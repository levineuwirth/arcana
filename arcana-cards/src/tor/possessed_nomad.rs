//! Possessed Nomad — `{2}{W}{W}` 3/3 Human Nomad Horror.
//!
//! Vigilance.
//! Threshold — As long as there are seven or more cards in your graveyard,
//! this creature gets +1/+1, is black, and has
//! "{2}{B}, {T}: Destroy target white creature."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Possessed Nomad");
    let human = reg.interner_mut().intern("Human");
    let nomad = reg.interner_mut().intern("Nomad");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(nomad);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "Threshold — As long as there are seven or more cards in your
    // graveyard, this creature gets +1/+1, is black, and has '{2}{B}, {T}:
    // Destroy target white creature.'" — a graveyard-gated static continuous
    // ability granting P/T, color, and an activated ability; no triggered/
    // activated hook expresses a conditional static here.
    reg.register(CardDefinition::new(name, chars))
}
