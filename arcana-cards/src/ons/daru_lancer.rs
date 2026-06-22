//! Daru Lancer — `{4}{W}{W}` 3/4 white Human Soldier.
//! First strike.
//! Morph {2}{W}{W}.
//!
//! Abilities:
//!  - First strike → `KeywordAbility::FirstStrike`.
//!  - Morph: the morph (face-down cast) mechanic is NOT in the usable
//!    keyword surface for this card class → GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: Morph {2}{W}{W} — the morph (face-down cast / turn-face-up) mechanic
// is not expressible with the demonstrated keyword/effect surface.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daru Lancer");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
