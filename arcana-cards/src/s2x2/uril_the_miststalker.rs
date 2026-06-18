//! Uril, the Miststalker — `{2}{R}{G}{W}` 5/5 Legendary Beast with Hexproof.
//! "Uril gets +2/+2 for each Aura attached to it" is a static continuous
//! self-buff scaling with attached Auras — not expressible as a triggered or
//! activated ability, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Uril, the Miststalker");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    // GAP: static "Uril gets +2/+2 for each Aura attached to it" —
    // dynamic continuous self-pump not expressible here.
    reg.register(CardDefinition::new(name, chars))
}
