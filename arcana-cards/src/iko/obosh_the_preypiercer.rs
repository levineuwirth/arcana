//! Obosh, the Preypiercer — `{3}{B/R}{B/R}` 3/5 Legendary Hellion
//! Horror.
//! Companion — odd-mana-value-only deck.
//! Static: sources you control with an odd mana value deal double
//! damage.
//!
//! Companion is not in the usable keyword surface, and the
//! damage-doubling static replacement is a continuous/static ability
//! with no triggered/activated decomposition — both are GAP'd.
//! Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Obosh, the Preypiercer");
    let hellion = reg.interner_mut().intern("Hellion");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);
    subtypes.0.insert(horror);

    // GAP: Companion — not in the usable keyword surface.
    // GAP: static "sources you control with odd mana value deal double
    //      damage" — continuous damage-doubling replacement, no
    //      triggered/activated decomposition.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B/R}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
