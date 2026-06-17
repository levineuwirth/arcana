//! Worldheart Phoenix — `{3}{R}` 2/2 Phoenix.
//! Flying.
//! You may cast this card from your graveyard by paying {W}{U}{B}{R}{G}
//! rather than its mana cost; if you do, it enters with two +1/+1 counters.
//! (GAP: alternate graveyard-cast cost with enters-with-counters rider is not
//! expressible with the demonstrated API.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Worldheart Phoenix");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: alternate-cost cast from graveyard ({W}{U}{B}{R}{G}) + enters with
    // two +1/+1 counters — not expressible as a triggered/activated ability.

    reg.register(CardDefinition::new(name, chars))
}
