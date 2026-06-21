//! Khârn the Betrayer — `{3}{R}` 5/1 Legendary Astartes Berserker.
//!
//! All three abilities are unexpressible with the available API:
//! - GAP: "Berzerker — attacks or blocks each combat if able" — no
//!   must-attack/must-block static Effect or keyword.
//! - GAP: "Sigil of Corruption — When you lose control of Khârn, draw
//!   two cards" — no lose-control TriggerCondition variant.
//! - GAP: "The Betrayer — if damage would be dealt to Khârn, prevent it
//!   and an opponent gains control of it" — replacement + control swap
//!   not expressible as a static here.
//!
//! The Scryfall keyword line (Berzerker / Sigil of Corruption / The
//! Betrayer) is ability-word naming, not real keywords — keywords vec
//! stays empty.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Khârn the Betrayer");
    let astartes = reg.interner_mut().intern("Astartes");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
