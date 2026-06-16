//! Elgaud Shieldmate — `{3}{U}` 2/3 Human Soldier.
//!
//! Oracle:
//! Soulbond
//! As long as this creature is paired with another creature, both creatures
//! have hexproof.
//!
//! Decomposition: `Soulbond` is not in the usable `KeywordAbility` surface, so
//! `keywords: vec![]`. Its remaining text is a single pairing-conditioned
//! static continuous ability ("as long as … paired … both have hexproof"),
//! which is neither a triggered nor an activated ability and is not
//! expressible in the demonstrated API. The card therefore carries only its
//! bones; the static and the Soulbond mechanic are GAP'd below.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: Soulbond pairing mechanic is not in the usable keyword surface.
// GAP: static "as long as this creature is paired with another creature, both
// creatures have hexproof" — a pairing-conditioned continuous ability, not a
// triggered/activated ability and not expressible in the demonstrated API.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elgaud Shieldmate");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
