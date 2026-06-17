//! Sunscape Familiar — `{1}{W}` 0/3 white Wall with Defender.
//! "Green spells and blue spells you cast cost {1} less to cast." is a
//! static cost-reduction effect — not a triggered or activated ability
//! — and there is no demonstrated Effect / primitive for casting-cost
//! reduction, so that line is GAP'd. Defender is emitted as a keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunscape Familiar");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: static "Green spells and blue spells you cast cost {1} less"
    // — a casting-cost-reduction continuous effect, not a triggered or
    // activated ability and not expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
