//! Bellowing Tanglewurm — `{3}{G}{G}` 4/4 Wurm with Intimidate.
//!
//! Intimidate.
//! Other green creatures you control have intimidate.
//!
//! The keyword line is expressible; the anthem-style static granting
//! intimidate to other green creatures is a pure continuous static with no
//! trigger/activated form, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bellowing Tanglewurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };

    // GAP: "Other green creatures you control have intimidate." — pure
    // continuous keyword-granting static, not expressible as a triggered or
    // activated ability.
    reg.register(CardDefinition::new(name, chars))
}
