//! Nyxborn Unicorn — `{1}{W}` 2/2 Enchantment Creature — Unicorn with
//! Mentor.
//!
//! "Bestow {3}{W}. Mentor. Enchanted creature gets +2/+2 and has
//! mentor."
//!
//! Mentor is a base keyword (the engine wires its attack trigger). The
//! Bestow alternative cost and the bestowed-Aura static buff ("enchanted
//! creature gets +2/+2 and has mentor") are GAP'd — Bestow is not a
//! demonstrated keyword variant and the attached-buff static is not a
//! triggered/activated ability for this card class.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nyxborn Unicorn");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Mentor],
        ..Default::default()
    };

    // GAP: Bestow {3}{W} (alternative Aura cost) — not a demonstrated
    //      keyword variant.
    // GAP: "Enchanted creature gets +2/+2 and has mentor" — bestowed
    //      Aura static buff, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
