//! True-Name Nemesis — `{1}{U}{U}` 3/1 Creature — Merfolk Rogue.
//! As this creature enters, choose a player.
//! This creature has protection from the chosen player.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("True-Name Nemesis");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "As this creature enters, choose a player." is an as-enters
    // replacement that sets up a chosen-player designation; there is no
    // expressible primitive for it in this shape.
    // GAP: "This creature has protection from the chosen player." Protection
    // is not an available KeywordAbility variant, and protection-from-a-chosen
    // player is a parametrized static that cannot be expressed here.
    reg.register(CardDefinition::new(name, chars))
}
