//! Rarity — `{1}{W}{U}` 2/2 Legendary Unicorn.
//! Both abilities are GAP'd:
//!  - "Rare and mythic rare spells you cast cost {1} less" is a static cost
//!    reduction with no engine representation.
//!  - "{1}, {T}, Reveal a My Little Pony® toy you own: ... protection from
//!    each color in that toy's coat, mane, and outfit" is an Un-set
//!    physical-prop ability that cannot be expressed.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rarity");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
