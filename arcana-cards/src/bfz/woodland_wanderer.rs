//! Woodland Wanderer — `{3}{G}` 2/2 Elemental.
//! Vigilance, trample.
//! Converge — enters with a +1/+1 counter for each color of mana spent
//! to cast it. (GAP'd — no mana-color-spent accessor / enters-with
//! replacement keyed on colors spent is expressible.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Woodland Wanderer");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
        // GAP: Converge enters-with-counters (colors of mana spent) has
        // no expressible hook.
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
