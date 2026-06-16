//! Siren Reaver — `{3}{U}` 3/2 Siren Pirate with Flying.
//! "Raid — This spell costs {1} less to cast if you attacked this turn."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Siren Reaver");
    let siren = reg.interner_mut().intern("Siren");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siren);
    subtypes.0.insert(pirate);

    // GAP: "Raid — costs {1} less if you attacked this turn" — a static conditional
    //      cast-cost reduction; Raid is not an available KeywordAbility variant and
    //      cost reduction has no expressible primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
