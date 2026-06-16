//! Armguard Familiar — `{1}{U}` 2/1 Artifact Creature — Equipment Beast.
//! Ward {2}. The "Equipped creature gets +2/+1 and has ward {2}" static
//! and the Reconfigure {4} ability are GAP (Equipment-attach static and
//! Reconfigure are not in the usable surface for this card class).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Armguard Familiar");
    let equipment = reg.interner_mut().intern("Equipment");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    // GAP: "Equipped creature gets +2/+1 and has ward {2}." — Equipment
    // attach static, not an expressible triggered/activated ability.
    // GAP: Reconfigure {4} — not in the usable keyword surface.
    reg.register(CardDefinition::new(name, chars))
}
