//! Wildvine Pummeler — `{6}{G}` 6/5 Giant Berserker with Reach and Trample.
//! "Vivid — This spell costs {1} less to cast for each color among permanents
//!  you control.
//!  Reach, trample"
//!
//! Reach + Trample are base keywords. Vivid (a cast-time cost reduction) is not
//! a modeled keyword/effect — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wildvine Pummeler");
    let giant = reg.interner_mut().intern("Giant");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Vivid cost reduction (per-color cost discount) — not modeled.
    reg.register(CardDefinition::new(name, chars))
}
