//! Moritte of the Frost — `{2}{G}{U}{U}` 0/0 Legendary Snow Shapeshifter
//! with Changeling.
//! "You may have Moritte enter as a copy of a permanent you control, …"
//!   (enter-as-a-copy replacement effect — not expressible; GAP'd)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moritte of the Frost");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // Legendary + Snow supertypes.
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY | SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    // GAP: "You may have Moritte enter as a copy of a permanent you control,
    // except it's legendary and snow … and has changeling" is an
    // as-it-enters copy replacement effect — no expressible Effect/ability.

    reg.register(CardDefinition::new(name, chars))
}
