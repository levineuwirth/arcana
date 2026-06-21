//! Seton's Scout — `{1}{G}` 2/1 Centaur Druid Scout Archer.
//!
//! Reach.
//! Threshold — This creature gets +2/+2 as long as there are seven or more
//! cards in your graveyard.
//!
//! Reach is a base keyword. The Threshold static (a conditional continuous
//! +2/+2 buff gated on graveyard size) has no expressible primitive in the
//! multi-ability creature surface — it is a pure static, not a triggered or
//! activated ability — so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seton's Scout");
    let centaur = reg.interner_mut().intern("Centaur");
    let druid = reg.interner_mut().intern("Druid");
    let scout = reg.interner_mut().intern("Scout");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(druid);
    subtypes.0.insert(scout);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: Threshold static "+2/+2 as long as seven or more cards in your
    // graveyard" is a conditional continuous static buff, not a
    // triggered/activated ability — not expressible here.

    reg.register(CardDefinition::new(name, chars))
}
