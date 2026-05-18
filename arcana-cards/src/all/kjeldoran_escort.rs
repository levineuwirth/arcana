//! Kjeldoran Escort — `{2}{W}{W}` 2/3 Human Soldier with Banding.
//! Ice Age common (1995); a Kjeldoran banding creature representing
//! the disciplined military formations of the Kjeldoran army.
//!
//! # Rules references
//!
//! * CR 702.21 — Banding. Any creatures with banding, and up to one
//!   without, can attack in a band. Bands are blocked as a group. If
//!   any creatures with banding you control are blocking or being
//!   blocked by a creature, you divide that creature's combat damage,
//!   not its controller, among any of the creatures it's being blocked
//!   by or is blocking.
//!
//! Banding is a base characteristic on this card; the runtime combat
//! pipeline handles the mechanic once `KeywordAbility::Banding` is
//! present in `keywords`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kjeldoran Escort");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Banding],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
