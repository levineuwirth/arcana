//! Anara, Wolvid Familiar — `{3}{G}` 4/4 Legendary Creature — Wolf Beast.
//! During your turn, commanders you control have indestructible. (static — GAP)
//! Partner. (keyword not in usable surface — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anara, Wolvid Familiar");
    let wolf = reg.interner_mut().intern("Wolf");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Partner — keyword not in usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "During your turn, commanders you control have indestructible."
    // — a conditional continuous keyword-granting static is not expressible here.

    reg.register(CardDefinition::new(name, chars))
}
