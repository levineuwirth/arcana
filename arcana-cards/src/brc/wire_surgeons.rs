//! Wire Surgeons — `{4}{B}{B}` 6/5 Human Artificer.
//! Fear. The static "Each artifact creature card in your graveyard has encore.
//! Its encore cost is equal to its mana cost." is a global ability-granting static
//! over graveyard cards (Encore is not in the usable keyword surface and there's no
//! primitive to grant graveyard abilities) — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wire Surgeons");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Fear],
        ..Default::default()
    };

    // GAP: static granting Encore to artifact creature cards in your graveyard.
    reg.register(CardDefinition::new(name, chars))
}
