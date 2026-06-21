//! Plated Pegasus — `{2}{W}` 1/2 Pegasus.
//! Flash, Flying.
//! If a spell would deal damage to a permanent or player, prevent 1
//! damage that spell would deal to that permanent or player.
//!
//! The keyword line (Flash, Flying) is base characteristics. The
//! damage-prevention clause is a STATIC replacement effect keyed on a
//! SPELL source; `Effect::PreventDamageFrom` filters by an
//! `ObjectFilter` over permanents, which cannot select "a spell" as the
//! damage source, so that ability is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Plated Pegasus");
    let pegasus = reg.interner_mut().intern("Pegasus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pegasus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static replacement "prevent 1 damage a spell would deal" —
    // no Effect can install a continuous, spell-source-filtered
    // prevention shield.
    reg.register(CardDefinition::new(name, chars))
}
