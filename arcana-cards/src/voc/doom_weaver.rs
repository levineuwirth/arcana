//! Doom Weaver — `{4}{B}{B}` 1/8 Spider Horror with Reach.
//! Soulbond.
//! "As long as Doom Weaver is paired with another creature, each of those
//!  creatures has 'When this creature dies, draw cards equal to its power.'"
//!
//! GAP: Soulbond is not a supported KeywordAbility variant — omitted; the
//! pairing mechanic is unmodeled.
//! GAP (static): the paired-creatures dies-trigger grant is a conditional
//! continuous ability-granting static gated on the (unmodeled) pairing — no
//! expressible primitive — omitted. Only the Reach keyword is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doom Weaver");
    let spider = reg.interner_mut().intern("Spider");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
