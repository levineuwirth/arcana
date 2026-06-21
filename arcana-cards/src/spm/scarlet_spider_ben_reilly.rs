//! Scarlet Spider, Ben Reilly — `{1}{R}{G}` 4/3 Legendary Spider Human Hero
//! with Trample.
//!
//! Oracle:
//! * Web-slinging {R}{G}.
//! * Trample.
//! * Sensational Save — If Scarlet Spider was cast using web-slinging, he
//!   enters with X +1/+1 counters on him, where X is the mana value of the
//!   returned creature.
//!
//! Trample is a base characteristic. Web-slinging is an alternative-cast cost
//! (not a usable keyword, no alternative-cost primitive), and the Sensational
//! Save ETB depends on web-slinging cast information that the engine doesn't
//! expose — both are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scarlet Spider, Ben Reilly");
    let spider = reg.interner_mut().intern("Spider");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Web-slinging {R}{G} — alternative-cast cost; not a usable keyword and
    // no alternative-cost primitive.
    // GAP: "Sensational Save — enters with X +1/+1 counters where X is the mana
    // value of the returned creature" — depends on web-slinging cast data not
    // exposed by the engine.
    reg.register(CardDefinition::new(name, chars))
}
