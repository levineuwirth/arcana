//! Vug Lizard — `{1}{R}{R}` 3/4 Creature — Lizard.
//! Mountainwalk.
//! Echo {1}{R}{R}.
//!
//! Decomposition:
//! * keyword line → Mountainwalk → KeywordAbility::Landwalk("Mountain").
//! * Echo {1}{R}{R} → GAP: Echo is not among the supported
//!   KeywordAbility variants, and its "at the beginning of your
//!   upkeep, if this came under your control since the beginning of
//!   your last upkeep, sacrifice it unless you pay its echo cost"
//!   shape has no expressible "came under control since last upkeep"
//!   intervening-if predicate. The keyword is dropped rather than
//!   invent a variant; only Mountainwalk is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vug Lizard");
    let lizard = reg.interner_mut().intern("Lizard");
    let mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Landwalk(mountain)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
