//! Chorus of the Conclave — `{4}{G}{G}{W}{W}` 3/8 Legendary Dryad with
//! Forestwalk.
//! "As an additional cost to cast creature spells, you may pay any amount of
//!  mana. If you do, that creature enters with that many additional +1/+1
//!  counters on it."
//!
//! GAP (static): the cast-modifying additional-cost static (pay any amount of
//! mana → that many +1/+1 counters) is a continuous spell-cost ability with no
//! triggered/activated expression and no documented effect surface.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chorus of the Conclave");
    let dryad = reg.interner_mut().intern("Dryad");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Landwalk(forest)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
