//! Bonepicker Skirge — `{2}{B}` 2/2 Phyrexian Imp with Flying.
//! "Corrupted — As long as an opponent has three or more poison counters, this
//!  creature has deathtouch and lifelink."
//!
//! Corrupted is a poison-gated continuous static keyword grant with no trigger
//! or activation cost — not expressible in this card class — so it is GAP'd.
//! Flying is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bonepicker Skirge");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let imp = reg.interner_mut().intern("Imp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(imp);

    // GAP: "Corrupted — As long as an opponent has three or more poison
    // counters, this creature has deathtouch and lifelink." — a poison-gated
    // continuous static keyword grant, no trigger/activation hook.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
