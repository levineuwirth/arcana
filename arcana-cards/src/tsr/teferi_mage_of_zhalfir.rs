//! Teferi, Mage of Zhalfir — `{2}{U}{U}{U}` 3/4 Legendary Human Wizard.
//!
//! Oracle:
//! * Flash
//! * "Creature cards you own that aren't on the battlefield have flash."
//! * "Each opponent can cast spells only any time they could cast a
//!   sorcery."
//!
//! Flash is emitted faithfully. Both static abilities are GAP'd: the
//! demonstrated API has no continuous-static primitive for granting
//! flash to cards in other zones, nor for restricting opponents to
//! sorcery-speed casting.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teferi, Mage of Zhalfir");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash],
        // GAP: static "creature cards you own that aren't on the
        // battlefield have flash" — no off-battlefield keyword-grant
        // static in the demonstrated API.
        // GAP: static "each opponent can cast spells only any time they
        // could cast a sorcery" — no casting-speed restriction primitive.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
