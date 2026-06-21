//! Question Elemental? — `{2}{U}{U}` 3/4 Elemental?.
//! Did you know this creature has flying?
//! Are you aware that any time you say something that isn't a question, when a
//! player points out this fact first, they gain control of this creature?

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Question Elemental?");
    let elemental = reg.interner_mut().intern("Elemental?");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        // "Did you know this creature has flying?" — it has flying.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: the "say something that isn't a question / a player points it out →
    // they gain control" clause is an Un-set out-of-game social mechanic with no
    // expressible variant.
    reg.register(CardDefinition::new(name, chars))
}
