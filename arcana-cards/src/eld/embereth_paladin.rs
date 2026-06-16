//! Embereth Paladin — `{3}{R}` 4/1 Human Knight.
//! Haste.
//! Adamant — If at least three red mana was spent to cast this spell, this
//! creature enters with a +1/+1 counter on it.
//!
//! Adamant is not a usable `KeywordAbility` variant — keyword line is Haste
//! only. The Adamant rider (an enters-with-counter replacement gated on the
//! red mana spent to cast the spell) has no expressible hook: no
//! mana-spent accessor / intervening-if condition exists for "three red
//! mana was spent". GAP that clause.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Embereth Paladin");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
