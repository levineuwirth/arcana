//! Ember Weaver — `{2}{G}` 2/3 green Spider.
//!
//! Reach.
//! As long as you control a red permanent, this creature gets +1/+0
//! and has first strike.
//!
//! The conditional continuous buff (+1/+0 and first strike while you
//! control a red permanent) is a static layer effect with no
//! triggered/activated shape in the demonstrated API, so it is GAP'd.
//! Reach is fully expressed.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ember Weaver");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: static "as long as you control a red permanent, this creature
    // gets +1/+0 and has first strike" — conditional continuous buff with
    // no triggered/activated shape.
    reg.register(CardDefinition::new(name, chars))
}
