//! Aven Wind Guide — `{2}{W}{U}` 2/3 Bird Warrior with Flying and Vigilance.
//! The token-anthem static and Embalm are GAP'd (no continuous-anthem
//! primitive; Embalm not in the usable keyword surface).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aven Wind Guide");
    let bird = reg.interner_mut().intern("Bird");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(warrior);

    // GAP: static "Creature tokens you control have flying and vigilance" —
    // no continuous token-anthem primitive in the usable surface.
    // GAP: keyword Embalm {4}{W}{U} — not in the usable keyword surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
