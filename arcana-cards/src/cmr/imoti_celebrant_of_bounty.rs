//! Imoti, Celebrant of Bounty — `{3}{G}{U}` 3/1 Legendary Snake Druid.
//! Cascade.
//! Spells you cast with mana value 6 or greater have cascade. — GAP: granting
//! cascade to other spells (a continuous static) is not expressible.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Imoti, Celebrant of Bounty");
    let snake = reg.interner_mut().intern("Snake");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Cascade is a cast-trigger keyword; it is not in the implemented
        // KeywordAbility surface for this card class, and the static "Spells you
        // cast with mana value 6 or greater have cascade" grants cascade to
        // other spells — a continuous static that is also not expressible.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
