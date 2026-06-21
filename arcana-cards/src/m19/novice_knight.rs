//! Novice Knight — `{W}` 2/3 Human Knight with Defender.
//!
//! "Defender. As long as this creature is enchanted or equipped, it can
//! attack as though it didn't have defender."
//!
//! Defender is a base keyword. The conditional "can attack despite
//! defender while enchanted/equipped" static is a continuous attack-
//! restriction override with no primitive in this API surface — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Novice Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: static — "As long as this creature is enchanted or equipped,
    // it can attack as though it didn't have defender" (no conditional
    // attack-restriction override primitive in this API surface).
    reg.register(CardDefinition::new(name, chars))
}
