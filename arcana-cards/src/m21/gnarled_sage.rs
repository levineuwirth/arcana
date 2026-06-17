//! Gnarled Sage — `{3}{G}{G}` 4/4 Treefolk Druid with Reach.
//! "As long as you've drawn two or more cards this turn, this creature
//! gets +0/+2 and has vigilance." — a conditional static buff with no
//! trigger/cost; not expressible as a triggered/activated ability, GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnarled Sage");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: "As long as you've drawn two or more cards this turn, this
    // creature gets +0/+2 and has vigilance" — conditional static
    // continuous ability, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
