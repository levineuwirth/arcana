//! Drove of Elves — `{3}{G}` */* Elf with Hexproof.
//!
//! Oracle:
//! * Hexproof — keyword.
//! * Drove of Elves's power and toughness are each equal to the number of
//!   green permanents you control. — a characteristic-defining ability. No
//!   primitive sets base P/T from a dynamic board count as a static, so P/T
//!   are left as `*` / `*` (the CDA itself is GAP'd).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drove of Elves");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: CDA — P/T = number of green permanents you control.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
