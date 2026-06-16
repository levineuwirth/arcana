//! Guardian of the Great Conduit — `{3}{G}` 2/4 Elemental with Reach.
//! As long as you control a Nissa planeswalker, this creature gets
//! +2/+0 and has vigilance.
//!
//! GAP: the conditional continuous static (+2/+0 and vigilance while
//! you control a Nissa planeswalker) is a pure static ability with no
//! trigger or cost, not expressible with the demonstrated
//! triggered/activated-ability API. Only the Reach keyword is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guardian of the Great Conduit");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
