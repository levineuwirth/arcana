//! Grim Wanderer — `{1}{B}` 5/3 black Goblin Warlock.
//! Flash.
//! Tragic Backstory — Cast this spell only if a creature died this turn.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grim Wanderer");
    let goblin = reg.interner_mut().intern("Goblin");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        // GAP: "Tragic Backstory — Cast this spell only if a creature died this
        // turn." A cast-legality restriction (not a triggered/activated
        // ability); no demonstrated primitive gates spell castability on a
        // per-turn event.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
