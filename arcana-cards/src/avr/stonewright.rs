//! Stonewright — `{R}` 1/1 Human Shaman.
//!
//! * Soulbond — GAP: Soulbond is not in the usable keyword set; the
//!   pairing mechanic is unmodeled.
//! * "As long as Stonewright is paired with another creature, each of
//!   those creatures has '{R}: This creature gets +1/+0 until end of
//!   turn.'" GAP: a static, pairing-gated CONFERRED activated ability
//!   (an ability granted to other objects, conditioned on the soulbond
//!   link) — not expressible as a triggered/activated ability on this
//!   card with the usable surface.
//!
//! Emitted with faithful bones and no abilities.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stonewright");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Soulbond keyword (not in usable set).
        // GAP: pairing-gated conferred "{R}: +1/+0" activated ability on the paired creatures.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
