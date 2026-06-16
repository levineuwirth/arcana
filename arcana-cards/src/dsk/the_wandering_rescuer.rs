//! The Wandering Rescuer — `{3}{W}{W}` 3/4 Legendary Human Samurai Noble.
//! Flash, Double strike. (Convoke is an alternative-cost casting keyword not in
//! the usable KeywordAbility surface — GAP'd.)
//! Static: "Other tapped creatures you control have hexproof." — a continuous
//! static granting hexproof to a filtered set; not expressible with the
//! demonstrated triggered/activated primitives — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Wandering Rescuer");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);
    subtypes.0.insert(noble);

    // GAP: Convoke — alternative-cost casting keyword, not in usable KeywordAbility surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // GAP: "Other tapped creatures you control have hexproof." — continuous static
    // granting a keyword to a filtered set; not expressible as a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
