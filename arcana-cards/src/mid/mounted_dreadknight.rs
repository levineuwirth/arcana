//! Mounted Dreadknight — `{4}{R}` 5/4 Vampire Knight with Trample.
//! "This creature enters with a +1/+1 counter on it if an opponent lost life
//! this turn."
//!
//! Trample is a base characteristic. The conditional enters-with-a-counter
//! clause is an intervening-if ETB ("if an opponent lost life this turn"), and
//! the only documented intervening-if predicates are control/life/hand/
//! graveyard/counter checks — none expresses "an opponent lost life this turn",
//! so the whole conditional-counter ability is GAP'd rather than fired
//! unconditionally (which would be a materially wrong card).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mounted Dreadknight");
    let vampire = reg.interner_mut().intern("Vampire");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "enters with a +1/+1 counter if an opponent lost life this turn" —
    // the intervening-if gate (an-opponent-lost-life-this-turn) is not among the
    // documented conditions:: predicates, so the conditional ETB counter is omitted.
    reg.register(CardDefinition::new(name, chars))
}
