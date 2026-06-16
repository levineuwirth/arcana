//! Serra Ascendant — `{W}` 1/1 Human Monk with Lifelink.
//! "As long as you have 30 or more life, this creature gets +5/+5 and has
//!  flying."
//!
//! The life-gated static buff (+5/+5 and flying while you have 30+ life) is a
//! continuous static ability with no trigger or activation cost — not
//! expressible in this card class — so it is GAP'd. Lifelink is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serra Ascendant");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);

    // GAP: "As long as you have 30 or more life, this creature gets +5/+5 and
    // has flying." — a life-gated continuous static, no trigger/activation hook.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
