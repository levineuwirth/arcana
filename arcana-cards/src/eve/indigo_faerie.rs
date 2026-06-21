//! Indigo Faerie — `{1}{U}` 1/1 Faerie Wizard with Flying.
//!
//! Oracle:
//! * Flying.
//! * {U}: Target permanent becomes blue in addition to its other colors until
//!   end of turn.
//!   GAP: Effect::SetColor REPLACES the target's color set; there is no
//!   additive "becomes blue in addition to its other colors" effect, so this
//!   activated ability is omitted rather than emit a materially-wrong color
//!   replacement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Indigo Faerie");
    let faerie = reg.interner_mut().intern("Faerie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "{U}: Target permanent becomes blue in addition to its other colors"
    //      — only color-REPLACING SetColor exists, not additive.
    reg.register(CardDefinition::new(name, chars))
}
