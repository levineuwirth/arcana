//! Sue, Everlasting Dinosaur — `{3}{R}{G}{W}` 7/6 Legendary Creature — Dinosaur.
//! Trample. Each creature card in your graveyard has fossilize / Fossilize
//! {R}{G}{W} (GAP'd — Fossilize is a bespoke graveyard keyword with no
//! available primitive).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sue, Everlasting Dinosaur");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "Each creature card in your graveyard has fossilize. Its fossilize
    // cost is equal to its mana cost minus {2}." and "Fossilize {R}{G}{W}" —
    // Fossilize is a bespoke graveyard-recursion keyword (return as an artifact
    // with a finality counter); it has no available KeywordAbility or Effect
    // primitive and is granted to other cards, so both lines are omitted.
    reg.register(CardDefinition::new(name, chars))
}
