//! Spirespine — `{2}{G}` 4/1 Enchantment Creature — Beast.
//! Bestow {4}{G} — not in the usable keyword surface; GAP'd (the card stays a
//! plain enchantment creature; the as-an-Aura cast mode is not modeled).
//! "This creature blocks each combat if able." — a static must-block
//! restriction with no demonstrated hook; GAP'd.
//! "Enchanted creature gets +4/+1 and blocks each combat if able." — a bestow
//! Aura static; with bestow unmodeled and no must-block primitive, GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spirespine");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Bestow {4}{G} — not in the usable keyword surface.
        ..Default::default()
    };

    // GAP: static "This creature blocks each combat if able." — no must-block
    // primitive.
    // GAP: bestow Aura static "Enchanted creature gets +4/+1 and blocks each
    // combat if able." — bestow Aura mode + must-block not modeled.
    reg.register(CardDefinition::new(name, chars))
}
