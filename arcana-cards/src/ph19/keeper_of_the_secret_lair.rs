//! Keeper of the Secret Lair — `{1}{U}` 2/1 Legendary Merfolk Rogue.
//! Flash.
//! You may cast Secret Lair spells as though they had flash. (static — GAP)
//! Secret Lair spells you cast cost {1} less to cast. (static — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keeper of the Secret Lair");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(rogue);

    // GAP: static "You may cast Secret Lair spells as though they had flash."
    // — no static casting-permission primitive, and "Secret Lair" is not a
    // recognizable card characteristic.
    // GAP: static "Secret Lair spells you cast cost {1} less to cast." — no
    // static cost-reduction primitive in the demonstrated triggered/activated API.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
