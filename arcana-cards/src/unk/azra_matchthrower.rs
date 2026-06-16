//! Azra Matchthrower — `{3}{R}` 4/4 red Azra Warrior.
//! "Battlebond (pair with an unpaired battle you control …)"
//! "As long as Azra Matchthrower is paired with a battle, each of
//!  those permanents has 'Whenever this permanent is dealt combat
//!  damage, this permanent deals 1 damage to any target.'"
//!
//! Battlebond is not an expressible keyword here, and its sole rules
//! payload is a pairing-conditional static that grants a triggered
//! ability to the paired permanents — a continuous static, not a
//! triggered/activated ability, and not expressible with the
//! demonstrated API. Only the faithful bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Azra Matchthrower");
    let azra = reg.interner_mut().intern("Azra");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(azra);
    subtypes.0.insert(warrior);

    // GAP: keyword "Battlebond" is not in the usable keyword surface; emit none.
    // GAP: static "As long as ~ is paired with a battle, those permanents have
    //      '<triggered ability>'" — a pairing-conditional ability-granting
    //      static, not a triggered/activated ability and not expressible here.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
