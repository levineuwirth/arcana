//! Takeno, Samurai General — `{5}{W}` 3/3 legendary Human Samurai with
//! Bushido 2. "Each other Samurai creature you control gets +1/+1 for
//! each point of bushido it has."
//!
//! Bushido 2 wired. The Samurai anthem is a pure continuous ability
//! (no trigger/cost) and is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Takeno, Samurai General");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Bushido(2)],
        ..Default::default()
    };

    // GAP: static "Each other Samurai creature you control gets +1/+1
    // for each point of bushido it has" — a pure continuous anthem, not
    // a triggered/activated ability.

    reg.register(CardDefinition::new(name, chars))
}
