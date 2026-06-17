//! Triton Wavebreaker — `{U}` 1/1 Enchantment Creature — Merfolk Wizard.
//! Bestow {1}{U} (not an expressible keyword — GAP).
//! "As long as this permanent is a creature, it has prowess." (static — GAP)
//! "Enchanted creature gets +1/+1 and has prowess." (Aura bestow static — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Triton Wavebreaker");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Bestow is not in the supported keyword surface; Prowess is not a
        //      supported KeywordAbility variant either.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "while this permanent is a creature, it has prowess" — conditional
    //      static keyword grant; Prowess unsupported.
    // GAP: "enchanted creature gets +1/+1 and has prowess" — Aura static; bestow
    //      not modeled.
    reg.register(CardDefinition::new(name, chars))
}
