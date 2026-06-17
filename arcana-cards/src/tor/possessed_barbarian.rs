//! Possessed Barbarian — `{2}{R}{R}` 3/3 red Human Barbarian Horror.
//! First strike. Threshold: while seven or more cards are in your graveyard, it
//! gets +1/+1, is black, and gains "{2}{B}, {T}: Destroy target red creature."
//!
//! Only the keyword line is expressible; the Threshold clause is a single
//! static continuous ability (conditional pump + recolor + granted activated
//! ability) with no triggered/activated form here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Possessed Barbarian");
    let human = reg.interner_mut().intern("Human");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(barbarian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: Threshold static — conditional +1/+1, color change, and granted
    // activated ability is one static continuous ability, not expressible here.
    reg.register(CardDefinition::new(name, chars))
}
