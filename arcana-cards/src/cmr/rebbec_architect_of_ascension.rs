//! Rebbec, Architect of Ascension — `{3}{W}` 3/4 Legendary Human Artificer.
//!
//! Oracle:
//! * Artifacts you control have protection from each mana value among
//!   artifacts you control.
//! * Partner.
//!
//! The protection-granting anthem (dynamic protection-from-mana-value) has no
//! primitive and is GAP'd. Partner is not in the usable `KeywordAbility` set
//! and is GAP'd. Only the bones survive.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rebbec, Architect of Ascension");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    // GAP: static — "Artifacts you control have protection from each mana value
    // among artifacts you control" (dynamic protection anthem).
    // GAP: keyword — Partner is not in the usable KeywordAbility set.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
