//! Sky-Blessed Samurai — `{6}{W}` 4/4 Enchantment Creature — Human Samurai.
//! Affinity for enchantments (cost reduction); Flying.
//!
//! Flying is a base keyword. Affinity for enchantments is a cost-reduction
//! keyword not in the usable keyword surface for this card class — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sky-Blessed Samurai");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    // GAP: Affinity for enchantments — cost-reduction keyword not in usable surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
