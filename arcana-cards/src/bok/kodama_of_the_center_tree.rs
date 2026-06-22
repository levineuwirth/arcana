//! Kodama of the Center Tree — `{4}{G}` */* Legendary Spirit.
//!
//! Oracle:
//! * Kodama's power and toughness are each equal to the number of Spirits
//!   you control.
//! * Kodama has soulshift X, where X is the number of Spirits you control.
//!
//! The bones P/T are transcribed as `Star` / `Star` (`*`/`*`).
//!
//! GAP (CDA): "power and toughness are each equal to the number of Spirits
//! you control" — the characteristic-defining computation is not wired; the
//! base `*` values are recorded.
//! GAP (keyword): "soulshift X, where X is the number of Spirits you
//! control" — `KeywordAbility::Soulshift(N)` requires a fixed integer N,
//! but here X is dynamic (a board count), which is not expressible.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kodama of the Center Tree");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
