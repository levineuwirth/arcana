//! Adriana, Captain of the Guard — `{3}{R}{W}` 4/4 Legendary Human Knight.
//! Melee. Other creatures you control have melee.
//!
//! Melee is NOT among the engine's usable `KeywordAbility` variants, so it
//! cannot be listed in `keywords` (GAP). The "Other creatures you control have
//! melee" line is a static keyword-granting ability with no usable keyword to
//! grant and no trigger/cost to decompose into — also GAP'd. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

// GAP: keyword "Melee" is not in the engine's usable KeywordAbility surface;
// emit no keyword for it.
// GAP: static "Other creatures you control have melee." — a continuous
// keyword-grant with no expressible keyword and no trigger/cost to decompose.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adriana, Captain of the Guard");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
