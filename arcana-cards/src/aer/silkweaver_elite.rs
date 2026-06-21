//! Silkweaver Elite — `{2}{G}` 2/2 Elf Archer.
//! Reach.
//! Revolt — When this creature enters, if a permanent left the
//! battlefield under your control this turn, draw a card. — GAP.
//!
//! GAP: the Revolt intervening-if ("if a permanent left the battlefield
//! under your control this turn") has no conditions:: predicate — it
//! covers ANY permanent leaving from ANY zone, broader than the
//! available a_creature_died_this_turn. Firing the draw unconditionally
//! would be materially wrong, so the gated ability is GAP'd whole.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silkweaver Elite");
    let elf = reg.interner_mut().intern("Elf");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
