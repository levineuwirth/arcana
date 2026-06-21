//! Putrid Pals — `{2}{B/G}{B/G}` 3/3 Human Ooze Mutant.
//! Deathtouch.
//! Disappear — "This creature enters with two +1/+1 counters on it if a
//!  permanent left the battlefield under your control this turn."
//!
//! Deathtouch is wired. The conditional enters-with-counters clause is GAP'd:
//! its gate ("a permanent left the battlefield under your control this turn")
//! has no expressible intervening-if condition, and adding the counters
//! unconditionally would be materially wrong, so the whole clause is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Putrid Pals");
    let human = reg.interner_mut().intern("Human");
    let ooze = reg.interner_mut().intern("Ooze");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ooze);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: "enters with two +1/+1 counters if a permanent left the battlefield
    // under your control this turn" — no expressible intervening-if for
    // "a permanent left the battlefield this turn"; omitted rather than add
    // counters unconditionally.
    reg.register(CardDefinition::new(name, chars))
}
