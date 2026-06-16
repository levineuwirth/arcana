//! Living Conundrum — `{4}{U}` 2/5 Elemental with Hexproof.
//! If you would draw a card while your library is empty, skip that draw.
//! As long as your library is empty, this creature has base P/T 10/10 and
//! flying and vigilance. (Both statics — GAP'd.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Living Conundrum");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };
    // GAP: "skip the draw while library is empty" (a draw-replacement effect)
    // and the conditional static "while library empty, base P/T 10/10 with
    // flying and vigilance" are not expressible.
    reg.register(CardDefinition::new(name, chars))
}
