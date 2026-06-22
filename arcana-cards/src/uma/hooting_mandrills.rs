//! Hooting Mandrills — `{5}{G}` 4/4 green Ape. Trample.
//! "Delve. Trample."
//!
//! Only the Trample keyword is expressible.
//! GAP: Delve (exile graveyard cards to pay for {1} each while casting) is
//!      not part of the usable keyword surface — keyword omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hooting Mandrills");
    let ape = reg.interner_mut().intern("Ape");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ape);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
