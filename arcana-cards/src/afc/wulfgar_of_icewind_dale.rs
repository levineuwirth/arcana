//! Wulfgar of Icewind Dale — `{3}{R}{G}` 4/4 Legendary Human Barbarian.
//! Melee; attacking-trigger doubler (a creature you control attacking causes a
//! triggered ability of a permanent you control to trigger an additional time).
//!
//! Bones only: Melee is not a usable keyword, and the attack-trigger-doubling
//! replacement static has no expressible primitive.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wulfgar of Icewind Dale");
    let human = reg.interner_mut().intern("Human");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(barbarian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: keyword Melee — not a usable KeywordAbility variant.
        ..Default::default()
    };

    // GAP: static — "If a creature you control attacking causes a triggered
    // ability of a permanent you control to trigger, that ability triggers an
    // additional time" — no trigger-doubling replacement primitive.

    reg.register(CardDefinition::new(name, chars))
}
