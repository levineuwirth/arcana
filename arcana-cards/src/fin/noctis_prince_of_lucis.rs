//! Noctis, Prince of Lucis — `{1}{W}{U}{B}` 4/3 Legendary Human Noble.
//! "Lifelink.
//!  You may cast artifact spells from your graveyard by paying 3 life in
//!  addition to paying their other costs. If you cast a spell this way, that
//!  artifact enters with a finality counter on it."
//!
//! Lifelink is a base keyword. The graveyard-cast-permission static is a
//! continuous ability with no trigger or activation cost — not expressible as
//! a TriggeredAbilityDef / ActivatedAbilityDef, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Noctis, Prince of Lucis");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        // GAP: "cast artifact spells from your graveyard for 3 extra life;
        // they enter with a finality counter" — a static cast-permission with
        // no trigger/activation hook; not expressible in this card class.
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
