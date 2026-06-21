//! Danitha, New Benalia's Light — `{1}{G}{W}` 2/2 Legendary Human Knight
//! with Vigilance, Trample, and Lifelink. "Once during each of your turns,
//! you may cast an Aura or Equipment spell from your graveyard." is a
//! cast-from-graveyard static permission with no demonstrated primitive,
//! so it is GAP'd; the keywords are base.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Danitha, New Benalia's Light");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    // GAP: "Once during each of your turns, you may cast an Aura or
    // Equipment spell from your graveyard" — cast-from-graveyard static
    // permission, not expressible.
    reg.register(CardDefinition::new(name, chars))
}
