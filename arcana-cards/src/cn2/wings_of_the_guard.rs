//! Wings of the Guard — `{1}{W}` 1/1 Bird with Flying and Melee.
//! Melee ("Whenever this attacks, it gets +1/+1 for each opponent you
//! attacked this combat") is not in the usable keyword surface and the
//! per-opponent-attacked count is not an available script helper.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wings of the Guard");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: Melee — not in the usable KeywordAbility surface, and "for each
    // opponent you attacked this combat" has no script helper.
    reg.register(CardDefinition::new(name, chars))
}
