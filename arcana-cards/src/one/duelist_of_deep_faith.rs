//! Duelist of Deep Faith — `{1}{W}` 2/2 Phyrexian Soldier with Toxic 1.
//! During your turn, this creature has first strike. (conditional static — GAP'd)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duelist of Deep Faith");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Toxic(1)],
        ..Default::default()
    };

    // GAP: "During your turn, this creature has first strike" — a conditional continuous
    //      static, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
