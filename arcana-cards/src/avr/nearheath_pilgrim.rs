//! Nearheath Pilgrim — `{1}{W}` 2/1 Human Cleric.
//! Soulbond. "As long as this creature is paired with another creature,
//!  both creatures have lifelink."
//!
//! Soulbond is not an available KeywordAbility variant and the pairing
//! machinery is unmodeled; the paired-lifelink static likewise has no
//! decomposition. Both are GAP'd — only bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: Soulbond (no KeywordAbility variant; pairing unmodeled).
// GAP: "while paired, both creatures have lifelink" — a paired-conditional
// static granting a keyword to two creatures, not expressible.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nearheath Pilgrim");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
