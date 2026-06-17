//! Morophon, the Boundless — `{7}` 6/6 Legendary Shapeshifter with Changeling.
//! "As Morophon enters, choose a creature type."
//! "Spells of the chosen type you cast cost {W}{U}{B}{R}{G} less to cast."
//! "Other creatures you control of the chosen type get +1/+1."
//!
//! All non-keyword text is static (a chosen-type cost reduction and a chosen-
//! type anthem); neither is a triggered or activated ability, so only the
//! Changeling keyword and the bones are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Morophon, the Boundless");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };
    // GAP static: "As Morophon enters, choose a creature type" + chosen-type
    // cost reduction + chosen-type anthem are continuous/replacement statics,
    // not triggered or activated abilities.
    reg.register(CardDefinition::new(name, chars))
}
