//! Cairn Wanderer — `{4}{B}` 4/4 Shapeshifter with Changeling.
//! As long as a creature card with flying is in a graveyard, this
//! creature has flying — likewise for fear, first strike, double
//! strike, deathtouch, haste, landwalk, lifelink, protection, reach,
//! trample, shroud, and vigilance (static — GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cairn Wanderer");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    // GAP: the graveyard-conditional keyword-granting clause is a static
    // continuous effect that scans graveyards, not a triggered/activated
    // ability.
    reg.register(CardDefinition::new(name, chars))
}
