//! Multani, Yavimaya's Avatar — `{4}{G}{G}` 0/0 legendary Elemental Avatar
//! with Reach and Trample.
//! "Multani gets +1/+1 for each land you control and each land card in your
//! graveyard." (dynamic static P/T, GAP'd.)
//! "{1}{G}, Return two lands you control to their owner's hand: Return this
//! card from your graveyard to your hand." (return-lands cost unexpressible, GAP'd.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Multani, Yavimaya's Avatar");
    let elemental = reg.interner_mut().intern("Elemental");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static "+1/+1 for each land you control and each land card in your
    //      graveyard" — dynamic continuous P/T from board+graveyard, no primitive.
    // GAP: "{1}{G}, Return two lands you control to their owner's hand: ..." —
    //      the return-two-lands activation cost is not an expressible ActivationCost.
    reg.register(CardDefinition::new(name, chars))
}
