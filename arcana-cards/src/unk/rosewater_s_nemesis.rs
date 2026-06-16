//! Rosewater's Nemesis — `{3}{W}{W}` 4/6 Monkey Cleric with Vigilance.
//! Protection from Phyrexians is not a usable keyword (gapped), and
//! "Poison Tolerance +3" is a static with no primitive (gapped).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rosewater's Nemesis");
    let monkey = reg.interner_mut().intern("Monkey");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(monkey);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: "protection from Phyrexians" — Protection is not a usable
        // keyword. Only Vigilance is emitted.
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "Poison Tolerance +3" — no poison-tolerance primitive.

    reg.register(CardDefinition::new(name, chars))
}
