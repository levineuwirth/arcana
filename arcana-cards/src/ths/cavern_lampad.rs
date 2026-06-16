//! Cavern Lampad — `{3}{B}` 2/2 Enchantment Creature — Nymph with Intimidate.
//! Bestow {5}{B}; "Enchanted creature gets +2/+2 and has intimidate."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cavern Lampad");
    let nymph = reg.interner_mut().intern("Nymph");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nymph);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Bestow is an alternative casting mode (not an available KeywordAbility variant).
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };
    // GAP: bestow Aura static "Enchanted creature gets +2/+2 and has intimidate"
    // is not expressible as a triggered/activated ability on this creature shape.
    reg.register(CardDefinition::new(name, chars))
}
