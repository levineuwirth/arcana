//! Villainous Ogre — `{2}{B}` 3/2 Ogre Warrior.
//! "This creature can't block." (static — GAP)
//! "As long as you control a Demon, this creature has '{B}: Regenerate this
//! creature.'" (conditional granted static — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Villainous Ogre");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);

    // GAP: "This creature can't block." — a static restriction with no
    // triggered/activated form (no ForbidBlocking-static primitive here).
    // GAP: "As long as you control a Demon, this creature has '{B}: Regenerate
    // this creature.'" — a conditional static that grants an activated ability;
    // not expressible as a plain ActivatedAbilityDef gated on board state.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
