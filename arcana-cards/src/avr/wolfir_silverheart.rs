//! Wolfir Silverheart — `{3}{G}{G}` 4/4 Wolf Warrior.
//! "Soulbond"
//! "As long as this creature is paired with another creature, each of
//! those creatures gets +4/+4."
//!
//! Soulbond is not in the usable keyword surface and is unmodeled (no
//! pairing mechanic). The "+4/+4 while paired" clause is a static
//! continuous buff gated on the soulbond pairing, which is likewise not
//! expressible; both are GAP'd. Only the bones remain.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: Soulbond — pairing keyword not in the usable KeywordAbility surface
// and no pairing mechanic is modeled.
// GAP: "As long as this creature is paired with another creature, each of
// those creatures gets +4/+4." — a static buff gated on soulbond pairing;
// not expressible (no pairing state, and statics have no trigger/cost).

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wolfir Silverheart");
    let wolf = reg.interner_mut().intern("Wolf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
