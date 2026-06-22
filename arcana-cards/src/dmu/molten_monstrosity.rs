//! Molten Monstrosity — `{7}{R}` 5/5 Hellion with Trample.
//! "This spell costs {X} less to cast, where X is the greatest power among
//! creatures you control." (a cost-reduction static — GAP'd)
//! "Trample"

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Molten Monstrosity");
    let hellion = reg.interner_mut().intern("Hellion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);

    // GAP: "This spell costs {X} less to cast, where X is the greatest
    // power among creatures you control" — a dynamic cost-reduction
    // static applied during casting; not expressible via a triggered/
    // activated ability and no cost-reduction Effect for spells.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
