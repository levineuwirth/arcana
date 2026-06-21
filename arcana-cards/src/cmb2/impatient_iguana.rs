//! Impatient Iguana — `{1}{R}` 2/1 Lizard Wizard with Haste.
//! "If Impatient Iguana is in your opening hand and you're not the
//!  starting player, you may reveal it. If you do, you become the
//!  starting player.
//!  Haste"

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Impatient Iguana");
    let lizard = reg.interner_mut().intern("Lizard");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(wizard);

    // GAP: "in your opening hand ... you become the starting player" — an
    // opening-hand reveal ability with no trigger/cost; not expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
