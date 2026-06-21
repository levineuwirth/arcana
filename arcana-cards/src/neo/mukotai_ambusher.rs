//! Mukotai Ambusher — `{3}{B}` 3/2 Artifact Creature — Rat Ninja.
//! Ninjutsu {1}{B}. (GAP — Ninjutsu is not an expressible KeywordAbility
//! variant, and its alternate-cost "return an unblocked attacker, put
//! this onto the battlefield" mechanic has no cost/effect primitive.)
//! Lifelink.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mukotai Ambusher");
    let rat = reg.interner_mut().intern("Rat");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Ninjutsu {1}{B} is not an expressible KeywordAbility variant.
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
