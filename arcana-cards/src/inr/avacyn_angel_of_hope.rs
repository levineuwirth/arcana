//! Avacyn, Angel of Hope — `{5}{W}{W}{W}` 8/8 Legendary Angel.
//! "Flying, vigilance, indestructible"
//! "Other permanents you control have indestructible."
//!
//! The three keywords are wired. The static "other permanents you control
//! have indestructible" is a continuous keyword-granting static with no
//! trigger or activation cost; it is not expressible as a
//! triggered/activated ability and is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

// GAP: "Other permanents you control have indestructible." — a static
// continuous keyword-granting ability with no trigger/cost; not
// expressible in the MultiAbilityCreature surface.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avacyn, Angel of Hope");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Indestructible,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
