//! Loyal Unicorn — `{3}{W}` 3/4 Unicorn.
//!
//! "Vigilance
//!  Lieutenant — At the beginning of combat on your turn, if you control
//!  your commander, prevent all combat damage that would be dealt to
//!  creatures you control this turn. Other creatures you control gain
//!  vigilance until end of turn."
//!
//! Decomposition: Vigilance keyword. The Lieutenant trigger is gated by
//! an intervening-if ("if you control your commander") that has no engine
//! condition helper. Since firing the ability unconditionally would be
//! materially wrong (it would prevent all combat damage to your creatures
//! every combat regardless of commander), the entire triggered ability is
//! GAP'd rather than fired without the gate.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Loyal Unicorn");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    // GAP: "Lieutenant — At the beginning of combat on your turn, if you
    // control your commander, …" — the intervening-if ("control your
    // commander") has no engine condition; firing unconditionally would
    // be materially wrong, so the whole triggered ability is omitted.
    reg.register(CardDefinition::new(name, chars))
}
