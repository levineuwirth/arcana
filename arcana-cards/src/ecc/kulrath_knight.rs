//! Kulrath Knight — `{3}{B/R}{B/R}` 3/3 Elemental Knight with Flying and Wither.
//! "Creatures your opponents control with counters on them can't attack or
//! block." (static combat restriction — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kulrath Knight");
    let elemental = reg.interner_mut().intern("Elemental");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B/R}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Wither],
        ..Default::default()
    };
    // GAP: "Creatures your opponents control with counters on them can't attack
    // or block." — static board-wide combat restriction, not expressible as a
    // triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
