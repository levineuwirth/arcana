//! Warded Battlements — `{2}{W}` 0/3 Wall with Defender.
//!
//! Oracle:
//! * Defender.
//! * Attacking creatures you control get +1/+0.  (static continuous anthem)
//!
//! Defender is a base characteristic. The anthem is a pure static
//! continuous effect with no trigger word and no cost — it is not a
//! triggered or activated ability and cannot be expressed via the
//! demonstrated ability-decomposition primitives, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warded Battlements");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: "Attacking creatures you control get +1/+0." — a pure static
    // continuous anthem (no trigger word, no cost); not expressible as a
    // triggered/activated ability with the demonstrated primitives.
    reg.register(CardDefinition::new(name, chars))
}
