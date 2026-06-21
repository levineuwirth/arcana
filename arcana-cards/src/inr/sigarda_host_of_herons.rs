//! Sigarda, Host of Herons — `{2}{G}{W}{W}` 5/5 Legendary Creature — Angel.
//!
//! * Flying, hexproof (keywords).
//! * "Spells and abilities your opponents control can't cause you to sacrifice
//!   permanents." — a static replacement/prohibition with no trigger word and
//!   no activation cost. There is no sacrifice-prohibition Effect primitive,
//!   so this static is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sigarda, Host of Herons");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Hexproof],
        ..Default::default()
    };
    // GAP: "Spells and abilities your opponents control can't cause you to
    // sacrifice permanents" — a continuous prohibition with no
    // triggered/activated form and no matching Effect primitive.
    reg.register(CardDefinition::new(name, chars))
}
