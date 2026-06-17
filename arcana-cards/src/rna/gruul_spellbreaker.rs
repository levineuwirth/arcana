//! Gruul Spellbreaker — `{1}{R}{G}` 3/3 Ogre Warrior with Riot and
//! Trample.
//! "During your turn, you and this creature have hexproof."
//!
//! Riot + Trample are base keywords. The "during your turn, you and
//! this creature have hexproof" conditional static has no expressible
//! primitive — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gruul Spellbreaker");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Riot],
        ..Default::default()
    };

    // GAP: static "during your turn, you and this creature have hexproof"
    // — no turn-conditional hexproof-granting primitive.
    reg.register(CardDefinition::new(name, chars))
}
