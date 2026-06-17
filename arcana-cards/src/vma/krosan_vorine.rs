//! Krosan Vorine — `{3}{G}` 3/2 Cat Beast with Provoke.
//! "This creature can't be blocked by more than one creature." (static, GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Krosan Vorine");
    let cat = reg.interner_mut().intern("Cat");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Provoke],
        ..Default::default()
    };

    // GAP: "This creature can't be blocked by more than one creature." —
    // a continuous blocking-restriction static, not a triggered/activated
    // ability and not expressible with the available Effect surface.

    reg.register(CardDefinition::new(name, chars))
}
