//! Towering Gibbon — `{3}{G}` */4 Ape with Reach.
//! "Towering Gibbon's power is equal to the greatest mana value among creatures
//! you control."
//!
//! GAP: the characteristic-defining static that sets power to the greatest mana
//! value among creatures you control has no expressible form (only
//! `PtValue::Fixed` is available — no variable/CDA power), so the printed `*`
//! power is recorded as `Fixed(0)` and the dynamic-power static is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Towering Gibbon");
    let ape = reg.interner_mut().intern("Ape");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ape);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
