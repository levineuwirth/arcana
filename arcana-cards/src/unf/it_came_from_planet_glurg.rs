//! It Came from Planet Glurg — `{X}{X}{G}{U}` 0/0 Legendary Creature — Alien Ooze.
//! "You may have It Came from Planet Glurg enter as a copy of X different
//! creatures on the battlefield." — an enters-as-a-copy(-of-multiple)
//! replacement; the copy-on-enter shape (and the X-different-creatures merge)
//! has no demonstrated hook, so it is GAP'd and the card is left as a vanilla
//! 0/0.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("It Came from Planet Glurg");
    let alien = reg.interner_mut().intern("Alien");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{X}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "may enter as a copy of X different creatures on the battlefield" —
    // an enters-as-a-copy replacement (merging multiple creatures) is not
    // expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
