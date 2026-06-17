//! The Playful Winners — `{3}{U}` 2/2 Legendary Human Gamer.
//! Enters as a copy of a random tournament-legal card with partner (copy-on-
//!   enter: GAP). If you win a game, put a tally mark on it (game-win trigger:
//!   GAP). Gets +2/+2 for each tally mark on it (static: GAP).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Playful Winners");
    let human = reg.interner_mut().intern("Human");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(gamer);

    // GAP: "enters as a copy of a random tournament-legal card with partner" —
    //      no copy-on-enter / random-card primitive.
    // GAP: "If you win a game ... put a tally mark" — no game-win trigger.
    // GAP: "gets +2/+2 for each tally mark" — conditional continuous static.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
