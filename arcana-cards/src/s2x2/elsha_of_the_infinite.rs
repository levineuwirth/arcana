//! Elsha of the Infinite — `{2}{U}{R}{W}` 3/3 Legendary Djinn Monk.
//!
//! All three non-bones abilities are GAP'd:
//! * "Prowess" — not in the KeywordAbility surface (keyword line empty).
//! * "You may look at the top card of your library any time." — a static
//!   information permission with no expressible primitive.
//! * "You may cast noncreature spells from the top of your library; if you
//!   cast a spell this way, you may cast it as though it had flash." — a
//!   static play-from-top permission with no expressible primitive.
//!
//! Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elsha of the Infinite");
    let djinn = reg.interner_mut().intern("Djinn");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    subtypes.0.insert(monk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
