//! Bad Knight — `{B}{B}` 2/2 Phyrexian Knight (team / multiplayer-variant
//! card). First Strike. The "can't cast on the Mirran team" and "score a
//! point for the Phyrexian team at end of game" clauses are team-variant
//! statics with no engine analog — GAP'd. Keyword + bones only.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bad Knight");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };
    // GAP: "You can't cast Bad Knight if you're on the Mirran team" — team
    // cast restriction, no engine analog.
    // GAP: "If Bad Knight is on the battlefield at the end of the game, score
    // an additional point for the Phyrexian team" — team scoring, no analog.
    reg.register(CardDefinition::new(name, chars))
}
