//! Locthwain Paladin — `{3}{B}` 3/2 Human Knight with Menace. Adamant — if at
//! least three black mana was spent to cast it, it enters with a +1/+1 counter.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Locthwain Paladin");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: Adamant — "if at least three black mana was spent to cast this spell,
    // this creature enters with a +1/+1 counter on it" — a cast-mana-dependent
    // enters-with clause, not expressible (Adamant is not a usable keyword and
    // the mana-spent condition cannot be inspected).
    reg.register(CardDefinition::new(name, chars))
}
