//! Sphinx of New Prahv — `{W}{W}{U}{U}` 4/3 Sphinx with Flying and Vigilance.
//! "Spells your opponents cast that target this creature cost {2} more to
//!  cast."
//!
//! The cost-increase is a static cost-modification ability, not a
//! triggered/activated ability and not expressible here. Bones + keywords.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sphinx of New Prahv");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}{U}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "Spells your opponents cast that target this creature cost {2} more
    // to cast" — static cost-modification; not expressible.
    reg.register(CardDefinition::new(name, chars))
}
