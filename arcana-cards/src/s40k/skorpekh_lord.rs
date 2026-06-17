//! Skorpekh Lord — `{2}{B}` 3/2 black Artifact Creature — Necron Noble
//! with Menace. "Command Protocols — Other artifact creatures you
//! control get +1/+0 and have menace." is a static anthem (filtered
//! continuous P/T + keyword grant), not a triggered or activated
//! ability, so it is GAP'd. "Unearth {2}{B}" is a graveyard alternate-
//! cast mechanic with no demonstrated primitive (there is no self-
//! reanimate-from-graveyard activated form in the available API), so it
//! is GAP'd as well. Menace is emitted as a keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skorpekh Lord");
    let necron = reg.interner_mut().intern("Necron");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "Command Protocols — Other artifact creatures you control get
    // +1/+0 and have menace." — a filtered continuous anthem, not a
    // triggered or activated ability.
    // GAP: "Unearth {2}{B}" — graveyard alternate-cast self-reanimate
    // mechanic with no demonstrated primitive.
    reg.register(CardDefinition::new(name, chars))
}
