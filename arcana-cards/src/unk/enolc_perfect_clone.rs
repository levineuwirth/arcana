//! Enolc, Perfect Clone — `*/*` Legendary Creature — Shapeshifter, no
//! mana cost (a commander-only card).
//!
//! Oracle text:
//! * If Enolc is one of two partner commanders, it's a copy of your
//!   other commander except it retains its name. (Including mana cost.)
//! * If Enolc isn't a commander, you may cast it as though it was a
//!   copy of any of your commanders.
//! * Partner.
//!
//! Implemented: the bones only (Legendary `*/*` Shapeshifter).
//!
//! GAP: Partner is not in the engine's `KeywordAbility` surface —
//! omitted.
//! GAP: both remaining abilities are commander-zone copy effects
//! (CR 903 / Partner-commander interactions) with no `Effect` or static
//! representation in the engine — omitted.

use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enolc, Perfect Clone");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
