//! The Horizon Seeker — `{W}{U}{B}{R}{G}` 0/0 Legendary Human Gamer.
//! Horsemanship.
//! The Horizon Seeker gets +1/+1 for each different named keyword and
//! ability word among permanents you control and instant or sorcery
//! cards in your graveyard.
//!
//! Horsemanship is a base keyword. The dynamic +1/+1-per-distinct-named-
//! keyword static is not expressible: there is no script helper that
//! counts distinct keyword/ability-word names across permanents and the
//! graveyard, and no continuous self-pump primitive driven by such a
//! count — so the static buff is a GAP. The card is emitted as its
//! printed 0/0 base with Horsemanship.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Horizon Seeker");
    let human = reg.interner_mut().intern("Human");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(gamer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Horsemanship],
        ..Default::default()
    };

    // GAP: "gets +1/+1 for each different named keyword and ability word
    // among permanents you control and instant or sorcery cards in your
    // graveyard" — no script helper counts distinct keyword/ability-word
    // names, and no self-static dynamic pump primitive is available.
    reg.register(CardDefinition::new(name, chars))
}
