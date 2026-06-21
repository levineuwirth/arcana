//! Oozeavite — `{4}{G}` 0/0 green Ooze.
//!
//! Oracle:
//! * Enters with a vigilance, reach, trample, and hexproof counter on it.
//!   These are also +1/+1 counters.
//! * If Oozeavite would be dealt damage, instead create a 0/0 green Ooze
//!   creature token and move a counter from Oozeavite onto it. Then if
//!   Oozeavite has no counters on it, sacrifice it.
//! * `{3}{G}, {T}`: Move any number of counters from other creatures you
//!   control onto Oozeavite.
//!
//! Bones only: the keyword-counter ETB, the damage-replacement, and the
//! variable-count counter-move activation all need machinery the
//! demonstrated API does not expose (counter-ETB riders / damage
//! replacement effects / a "move any number of counters" picker), so
//! they are GAP'd. A faithful 0/0 Ooze body is registered.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oozeavite");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "enters with a vigilance, reach, trample, and hexproof counter
    //       on it; these are also +1/+1 counters" — no enters-with-keyword-
    //       counter rider primitive is available.
    // GAP: "if Oozeavite would be dealt damage, instead create a token and
    //       move a counter onto it, then sacrifice if no counters" — damage
    //       replacement with counter-move is not expressible.
    // GAP: "{3}{G}, {T}: Move any number of counters from other creatures
    //       you control onto Oozeavite" — Effect::MoveCounter is single-id;
    //       there is no variable-count counter-move picker.
    reg.register(CardDefinition::new(name, chars))
}
