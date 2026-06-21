//! Mistform Dreamer — `{2}{U}` 2/1 blue Illusion with Flying.
//! "{1}: This creature becomes the creature type of your choice until
//! end of turn." Changing a creature's SUBTYPE to a chosen type is not
//! expressible with the demonstrated effect primitives (only card-TYPE
//! and color changes exist), so the activated ability is GAP'd; Flying
//! is a base keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mistform Dreamer");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "{1}: This creature becomes the creature type of your choice
    // until end of turn" — there is no effect to set a chosen creature
    // SUBTYPE (Effect::AddType handles card types only).
    reg.register(CardDefinition::new(name, chars))
}
