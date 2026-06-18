//! Prison Barricade — `{1}{W}` 1/3 Wall with Defender.
//! Kicker {1}{W} and its kicked-conditional ETB rider (enters with a +1/+1
//! counter and "can attack as though it didn't have defender") are GAP'd:
//! Kicker is not in the usable keyword surface and the "was kicked" state is
//! not a predicate available here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prison Barricade");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Kicker {1}{W} — not in the usable keyword surface.
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: "If this creature was kicked, it enters with a +1/+1 counter and
    // 'can attack as though it didn't have defender'" — kicked-state ETB
    // rider not expressible here.
    reg.register(CardDefinition::new(name, chars))
}
