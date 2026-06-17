//! Dalek Squadron — `{2}{B}` 3/3 Artifact Creature — Dalek with Menace.
//! Myriad is not a usable keyword and its attack-trigger token copies are
//! unexpressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dalek Squadron");
    let dalek = reg.interner_mut().intern("Dalek");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dalek);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Myriad is not in the usable KeywordAbility surface.
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "Myriad — Whenever this creature attacks, for each opponent other
    // than defending player, create a tapped-and-attacking token copy; exile
    // them at end of combat" — per-opponent token copies are unexpressible.

    reg.register(CardDefinition::new(name, chars))
}
