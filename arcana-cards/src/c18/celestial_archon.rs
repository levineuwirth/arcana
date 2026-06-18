//! Celestial Archon — `{3}{W}{W}` 4/4 Enchantment Creature — Archon.
//! Flying, first strike. Bestow {5}{W}{W}; while attached, the enchanted
//! creature gets +4/+4 and has flying and first strike.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Celestial Archon");
    let archon = reg.interner_mut().intern("Archon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(archon);

    // GAP: "Bestow {5}{W}{W}" — Bestow is not an available KeywordAbility and
    // there is no bestow-cost builder in this surface.
    // GAP: static "Enchanted creature gets +4/+4 and has flying and first
    // strike" — the aura/bestow buff is a continuous static, not a
    // triggered/activated ability.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
