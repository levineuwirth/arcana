//! Avatar of Me — `{2}{U}{U}` */* blue Avatar (Unglued silver-border).
//! "This spell costs {1} more to cast for each ten years you've been
//! alive. Avatar of Me's power is equal to your height in feet and its
//! toughness is equal to your American shoe size. Avatar of Me is the
//! color of your eyes."
//!
//! Every line references a real-world property of the player and is not
//! expressible in the engine — there is no in-game state for the
//! caster's age, height, shoe size, or eye color. We transcribe the
//! authoritative bones (`*/*` via PtValue::Star, blue per the Colors
//! line) and GAP the three un-card characteristic-defining statics.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avatar of Me");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // P/T are `*` — no in-game CDA can read a player's height/shoe size.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        // GAP: static — cost increase per ten real years alive (not in-game state).
        // GAP: static — power = your height in feet (no CDA, real-world property).
        // GAP: static — toughness = your American shoe size (no CDA, real-world property).
        // GAP: static — color = the color of your eyes (no CDA, real-world property).
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
