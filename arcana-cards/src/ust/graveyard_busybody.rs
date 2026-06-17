//! Graveyard Busybody — `{4}{U}{U}` */* Human Spy.
//! "All graveyards are also your graveyards.
//!  Graveyard Busybody's power and toughness are each equal to the number of
//!  cards with flavor text in your graveyards."
//!
//! Both abilities are statics with no primitive: the graveyard-sharing static
//! and the */* characteristic-defining ability counting flavor-text cards.
//! Bones (`*`/`*` P/T) are transcribed via `PtValue::Star`; both statics GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Graveyard Busybody");
    let human = reg.interner_mut().intern("Human");
    let spy = reg.interner_mut().intern("Spy");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spy);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    // GAP: static "All graveyards are also your graveyards" — no graveyard
    // ownership-sharing primitive.
    // GAP: CDA "*/* equal to the number of cards with flavor text in your
    // graveyards" — no flavor-text-counting characteristic-defining ability.
    reg.register(CardDefinition::new(name, chars))
}
