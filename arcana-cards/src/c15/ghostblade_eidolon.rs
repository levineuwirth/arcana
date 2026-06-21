//! Ghostblade Eidolon — `{2}{W}` 1/1 Enchantment Creature — Spirit with
//! Double strike.
//! "Bestow {5}{W}" (aura-cast mechanic — not a supported keyword; GAP).
//! "Enchanted creature gets +1/+1 and has double strike." (bestow aura
//! static — GAP; not expressible on this creature card class).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ghostblade Eidolon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };
    // GAP: "Bestow {5}{W}" + the bestow aura static "Enchanted creature gets
    // +1/+1 and has double strike" are not expressible on this creature class.
    reg.register(CardDefinition::new(name, chars))
}
