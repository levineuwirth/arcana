//! Windwright Mage — `{W}{U}{B}` 2/2 Artifact Creature — Human Wizard.
//! Lifelink.
//! This creature has flying as long as an artifact card is in your graveyard
//! (a conditional static keyword grant — GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Windwright Mage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: "has flying as long as an artifact card is in your graveyard" — a
    // conditional continuous keyword-granting static (no trigger / no cost) is
    // not expressible via the triggered/activated ability surface.
    reg.register(CardDefinition::new(name, chars))
}
