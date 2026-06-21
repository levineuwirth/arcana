//! Fen Hauler — `{6}{B}` 5/5 black Insect.
//!
//! * Improvise. — GAP: Improvise (a cast-time cost-reduction mechanic) is
//!   not in the usable `KeywordAbility` surface for this card class, so it
//!   is omitted (`keywords: vec![]`).
//! * This creature can't be blocked by artifact creatures. — GAP: this is
//!   a static, source-filtered evasion ability. `Effect::CantBeBlocked` is
//!   unconditional (no "by [filter]" form), so the filtered evasion can't
//!   be expressed; emitted as a vanilla 5/5.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fen Hauler");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
