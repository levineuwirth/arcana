//! A-Syndicate Infiltrator — `{2}{U}{B}` 3/3 black/blue Vampire Wizard.
//! Flying, ward {2}.
//! As long as there are five or more mana values among cards in your
//! graveyard, Syndicate Infiltrator gets +2/+2.
//!
//! Flying and Ward {2} are base keywords. The conditional "+2/+2 while 5+
//! mana values among cards in your graveyard" is a continuous static, not a
//! triggered/activated ability — not expressible here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Syndicate Infiltrator");
    let vampire = reg.interner_mut().intern("Vampire");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };
    // GAP: conditional static "+2/+2 while 5+ mana values among cards in your
    // graveyard" is a continuous self-buff, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
