//! Apex Devastator — `{8}{G}{G}` 10/10 Chimera Hydra with Cascade (x4).
//!
//! # Rules references
//!
//! * CR 702.84 — Cascade. When you cast this spell, exile cards from the top
//!   of your library until you exile a nonland card that costs less. You may
//!   cast it without paying its mana cost. Put the exiled cards on the bottom
//!   in a random order. Multiple instances each trigger separately.
//!
//! Cascade is not among the demonstrated `KeywordAbility` variants, so this
//! card is registered as a vanilla creature pending engine support. The verify
//! pipeline will flag the gap.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Apex Devastator");
    let chimera = reg.interner_mut().intern("Chimera");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chimera);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
