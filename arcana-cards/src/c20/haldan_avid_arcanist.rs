//! Haldan, Avid Arcanist — `{2}{U}` 1/4 Legendary Human Wizard.
//! Partner with Pako, and a static play-permission for exiled fetch-counter cards.
//! Both non-bones abilities are GAPs (see below).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haldan, Avid Arcanist");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "Partner with" / "Partner" not in the available KeywordAbility surface.
        ..Default::default()
    };

    // GAP: "Partner with Pako" ETB — "target player MAY put Pako into their
    // hand from their library, then shuffle." The optional, cross-player
    // tutor-by-name-to-a-target-player's-hand is not expressible with the
    // available Effect surface (TutorToHand acts on a fixed player and is
    // mandatory).
    //
    // GAP: "You may play lands and cast noncreature spells from among cards
    // you exiled that have fetch counters on them …" — a static play-from-
    // exile permission, not a triggered/activated ability.

    reg.register(CardDefinition::new(name, chars))
}
