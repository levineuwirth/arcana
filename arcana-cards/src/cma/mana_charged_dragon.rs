//! Mana-Charged Dragon — `{4}{R}{R}` 5/5 Dragon with Flying and Trample.
//! Join forces ("Whenever this attacks or blocks, each player starting
//! with you may pay any amount of mana; this gets +X/+0 where X is the
//! total paid") is not expressible — variable each-player mana payment
//! has no available cost/effect surface.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mana-Charged Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Join forces — "each player starting with you may pay any amount of
    //      mana; this gets +X/+0" — no variable each-player mana payment surface.
    reg.register(CardDefinition::new(name, chars))
}
