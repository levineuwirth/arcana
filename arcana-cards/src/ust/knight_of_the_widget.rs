//! Knight of the Widget — `{2}{W}` */* Artifact Creature — Cyborg Knight.
//! Vigilance. Power and toughness each equal to the number of Order of the Widget
//! watermarks among permanents you control. The watermark-counting CDA P/T is a
//! pure static characteristic-defining ability with no expressible primitive
//! (watermarks aren't a modeled game property) — GAP'd; base 0/0 placeholder.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of the Widget");
    let cyborg = reg.interner_mut().intern("Cyborg");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyborg);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        // GAP: P/T are each the number of Order of the Widget watermarks among
        // permanents you control — watermark-based CDA not modeled; placeholder 0/0.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
