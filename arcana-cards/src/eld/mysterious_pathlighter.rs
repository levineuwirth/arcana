//! Mysterious Pathlighter — `{2}{W}` 2/2 Faerie.
//!
//! Flying.
//! Each creature you control that has an Adventure enters with an
//! additional +1/+1 counter on it.
//!
//! The keyword (Flying) is a base characteristic. The second line is a
//! pure static replacement effect ("enters with an additional counter")
//! — it is neither a triggered nor an activated ability, and the
//! demonstrated MultiAbility primitives cannot express an enters-with
//! replacement, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mysterious Pathlighter");
    let faerie = reg.interner_mut().intern("Faerie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static replacement — "Each creature you control that has an
    // Adventure enters with an additional +1/+1 counter on it" is an
    // enters-with replacement effect, not a triggered/activated ability,
    // and is not expressible with the demonstrated primitives.
    reg.register(CardDefinition::new(name, chars))
}
