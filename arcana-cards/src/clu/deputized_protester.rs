//! Deputized Protester — `{2}{R}` 2/1 Human Warrior with Menace.
//! Melee (Whenever this creature attacks, it gets +1/+1 until end of turn
//! for each opponent you attacked this combat.)
//!
//! Menace is a base keyword. Melee is not in the usable keyword surface and
//! the "+1/+1 for each opponent you attacked this combat" amount has no
//! script helper — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deputized Protester");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    // GAP: Melee — not in the usable keyword surface; the "+1/+1 for each
    //      opponent you attacked this combat" amount has no script helper.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
