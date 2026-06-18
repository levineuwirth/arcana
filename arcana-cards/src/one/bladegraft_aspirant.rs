//! Bladegraft Aspirant — `{2}{R}` 2/3 Phyrexian Warrior with Menace.
//! Menace.
//! Equipment spells you cast cost {1} less to cast.
//! Activated abilities of Equipment you control that target this
//! creature cost {1} less to activate.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bladegraft Aspirant");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "Equipment spells you cast cost {1} less to cast." — static
    // spell-cost-reduction, no Effect/static expressible here.
    // GAP: "Activated abilities of Equipment you control that target this
    // creature cost {1} less to activate." — static ability-cost-reduction,
    // not expressible.
    reg.register(CardDefinition::new(name, chars))
}
