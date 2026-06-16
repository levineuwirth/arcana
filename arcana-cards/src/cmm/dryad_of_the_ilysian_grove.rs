//! Dryad of the Ilysian Grove — `{2}{G}` 2/4 Enchantment Creature —
//! Nymph Dryad. Both abilities are static and have no expressible
//! primitive, so this is bones-only.
//!
//! GAP: "You may play an additional land on each of your turns." —
//! static land-play modifier, no primitive.
//! GAP: "Lands you control are every basic land type in addition to
//! their other types." — static type-adding, no primitive.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dryad of the Ilysian Grove");
    let nymph = reg.interner_mut().intern("Nymph");
    let dryad = reg.interner_mut().intern("Dryad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nymph);
    subtypes.0.insert(dryad);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
