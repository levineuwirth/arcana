//! Mirror Wall — `{3}{U}` 3/4 Wall (blue) with Defender.
//! "{W}: This creature can attack this turn as though it didn't have
//!  defender."
//!
//! Defender is a base keyword. The activation grants a defender-bypass attack
//! permission, for which there is no expressible Effect → the activated
//! ability is GAP'd (no effect emitted); only the keyword bones are kept.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mirror Wall");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: "{W}: This creature can attack this turn as though it didn't have
    // defender." — no defender-bypass / can-attack permission Effect.
    reg.register(CardDefinition::new(name, chars))
}
