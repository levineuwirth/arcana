//! Reckless One — `{3}{R}` Goblin Avatar with Haste.
//! Power and toughness are each equal to the number of Goblins on the
//! battlefield (a characteristic-defining ability, modeled as a 0/0 base).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reckless One");
    let goblin = reg.interner_mut().intern("Goblin");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(avatar);

    // GAP: static CDA "power and toughness each equal to the number of
    // Goblins on the battlefield" — not expressible as a triggered/activated
    // ability; printed `*/*` modeled as a 0/0 base.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
