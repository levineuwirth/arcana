//! Chameleon, Master of Disguise — `{3}{U}` 2/3 Legendary Creature — Human
//! Shapeshifter Villain.
//!
//! * You may have Chameleon enter as a copy of a creature you control (except
//!   his name stays Chameleon, Master of Disguise).
//! * Mayhem {2}{U} (cast from graveyard if discarded this turn).
//!
//! Both lines are GAP'd. The "enter as a copy" is a copy-ETB replacement
//! effect (CR 706), not a triggered or activated ability — there is no
//! Effect/replacement primitive in the supported surface that mints the
//! creature as a copy at entry. Mayhem is an alternative-cost cast modifier
//! with no usable `KeywordAbility` variant. The card is registered with
//! faithful bones.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chameleon, Master of Disguise");
    let human = reg.interner_mut().intern("Human");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shapeshifter);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Mayhem {2}{U} — alternative cast cost, no usable keyword.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "You may have Chameleon enter as a copy of a creature you control" —
    // a copy-as-it-enters replacement effect (CR 706); not expressible as a
    // triggered or activated ability with the supported Effect surface.
    reg.register(CardDefinition::new(name, chars))
}
