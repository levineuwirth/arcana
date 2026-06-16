//! Ladies' Knight — `{3}{W}` 2/2 Human Knight with Flying.
//!
//! Oracle text:
//! * Flying.
//! * Spells cast by players wearing at least one item of women's clothing
//!   cost {1} less to cast. (Un-set joke static — out-of-game condition.)
//!
//! Only Flying is expressible. The cost-reduction static depends on an
//! out-of-game, physical condition ("wearing women's clothing"), which the
//! engine cannot model.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ladies' Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "Spells cast by players wearing at least one item of women's
    // clothing cost {1} less" — depends on an out-of-game physical condition;
    // not expressible with engine primitives.
    reg.register(CardDefinition::new(name, chars))
}
