//! Great Divide Guide — `{1}{G}` 2/3 green Human Scout Ally.
//! "Each land and Ally you control has '{T}: Add one mana of any color.'"
//! GAP: granting activated abilities to other permanents is not in the
//! Effect catalog; this is a static ability, not a triggered/activated one.
//! Emitting as a no-op creature (the static ability cannot be modeled).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Great Divide Guide");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);
    subtypes.0.insert(ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    // GAP: "each land and Ally you control has '{T}: Add one mana of any color'"
    // is a static ability that grants activated abilities — not expressible.
    reg.register(CardDefinition::new(name, chars))
}
