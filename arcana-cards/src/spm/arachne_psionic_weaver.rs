//! Arachne, Psionic Weaver — `{2}{W}` 3/3 Legendary Spider Human Hero.
//! "Web-slinging {W} (...)" — alternative cast cost; not expressible. GAP'd.
//! "As Arachne enters, look at an opponent's hand, then choose a card
//!  type other than creature." — a look-and-choose replacement with a
//!  stored choice; not expressible. GAP'd.
//! "Spells of the chosen type cost {1} more to cast." — static cost
//!  increase keyed to that stored choice; not expressible. GAP'd.
//!
//! All non-keyword text routes elsewhere; only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arachne, Psionic Weaver");
    let spider = reg.interner_mut().intern("Spider");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: Web-slinging {W} alternative cast cost — no alt-cost primitive.
    // GAP: "As ~ enters, look at an opponent's hand, then choose a card type" — look-and-store-choice ETB.
    // GAP: "Spells of the chosen type cost {1} more" — static cost-increase keyed to the stored choice.
    reg.register(CardDefinition::new(name, chars))
}
