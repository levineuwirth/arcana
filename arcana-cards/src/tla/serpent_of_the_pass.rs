//! Serpent of the Pass — `{5}{U}{U}` 6/5 Serpent.
//! "If there are three or more Lesson cards in your graveyard, you may cast
//! this spell as though it had flash." (conditional cast-timing permission — GAP)
//! "This spell costs {1} less to cast for each noncreature, nonland card in
//! your graveyard." (static cost reduction — GAP)
//! Both lines modify how the spell is cast; neither is a triggered/activated
//! ability, so the card is bones-only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serpent of the Pass");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
