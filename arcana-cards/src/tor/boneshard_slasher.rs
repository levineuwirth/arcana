//! Boneshard Slasher — `{1}{B}` 1/1 Horror with Flying.
//! "Threshold — As long as there are seven or more cards in your graveyard, this
//! creature gets +2/+2 and has 'When this creature becomes the target of a spell
//! or ability, sacrifice it.'" (static — GAP)
//!
//! GAP: the Threshold static (graveyard-gated +2/+2 and a granted becomes-target
//! sacrifice ability) is a conditional continuous effect with no expressible hook.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boneshard Slasher");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
